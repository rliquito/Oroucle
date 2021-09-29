use crate::system_utils::create_or_allocate_account_raw;
use crate::validation_utils::{
    assert_initialized, assert_is_ata, assert_keys_equal, assert_owned_by, assert_signer,
};
use crate::{
    error::RouletteError,
    instruction::RandomInstruction,
    state::{Guess, Honeypot, LockedGuess, RouletteGuess, Version, RNG},
};
use arrayref::array_refs;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    serialize_utils::read_u16,
    system_program, sysvar,
};
use spl_token::{
    instruction::{initialize_account, transfer},
    state::Account,
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = RandomInstruction::try_from_slice(instruction_data)?;
        match instruction {
            RandomInstruction::Initialize => {
                msg!("Instruction 0: Initialize");
                initialize(program_id, accounts)
            }
            RandomInstruction::Sample(args) => {
                msg!("Instruction 1: Sample");
                sample(accounts, args.tolerance)
            }
            RandomInstruction::InitializeHoneypot(args) => {
                msg!("Instruction 2: InitializeHoneypot");
                initialize_honeypot(
                    program_id,
                    accounts,
                    args.tick_size,
                    args.max_amount,
                    args.minimum_bank_size,
                )
            }
            RandomInstruction::WithdrawFromHoneypot(args) => {
                msg!("Instruction 3: WithdrawFromHoneypot");
                withdraw_from_honeypot(program_id, accounts, args.amount_to_withdraw)
            }
            RandomInstruction::InitializeGuessAccount => {
                msg!("Instruction 4: InitializeGuessAccount");
                initialize_guess_account(program_id, accounts)
            }
            RandomInstruction::PlaceGuesses(args) => {
                msg!("Instruction 5: PlaceGuesses");
                place_guesses(program_id, accounts, args.guesses)
            }
            RandomInstruction::Spin(args) => {
                msg!("Instruction 6: Spin");
                spin(program_id, accounts, args.tolerance)
            }
            RandomInstruction::TryCancel => {
                msg!("Instruction 7: TryCancel");
                try_cancel(program_id, accounts)
            }
        }
    }
}

fn initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let rng_account_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let rent_sysvar_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    if !rng_account_info.data_is_empty() {
        msg!("Received nonempty account");
        return Err(ProgramError::AccountAlreadyInitialized.into());
    }
    let (rng_key, rng_bump_seed) = Pubkey::find_program_address(
        &[b"random", payer_info.key.as_ref(), program_id.as_ref()],
        program_id,
    );
    let rng_seeds = &[
        b"random",
        payer_info.key.as_ref(),
        program_id.as_ref(),
        &[rng_bump_seed],
    ];
    if rng_key != *rng_account_info.key {
        msg!("RNG account doesn't match");
        return Err(ProgramError::InvalidArgument.into());
    }
    create_or_allocate_account_raw(
        rng_account_info,
        rent_sysvar_info,
        system_program_info,
        payer_info,
        program_id,
        RNG::LEN as usize,
        rng_seeds,
    )?;
    Ok(())
}

fn sample(accounts: &[AccountInfo], tolerance: u64) -> ProgramResult {
    let (rng_accounts, remaining_accounts) = array_refs![accounts, 1; .. ;];
    let (random_sample, slot) = random::random::sample(remaining_accounts, tolerance)?;
    let account_info_iter = &mut rng_accounts.iter();
    let rng_info = next_account_info(account_info_iter)?;
    let mut rng = RNG::from_account_info(rng_info)?;
    if rng.version == Version::Uninitalized {
        rng.version = Version::RNGV1;
    }
    rng.value = random_sample;
    rng.slot = slot;
    msg!("Sample {}", random_sample);
    rng.serialize(&mut *rng_info.data.borrow_mut())?;
    Ok(())
}

fn initialize_honeypot(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    tick_size: u64,
    max_amount: u64,
    minimum_bank_size: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let honeypot_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let vault_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_sysvar_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    msg!("Checking CPI program ID's");
    assert_keys_equal(system_program::id(), *system_program_info.key)?;
    assert_keys_equal(spl_token::id(), *token_program_info.key)?;
    msg!("Checking proper mint");
    assert_owned_by(mint_info, token_program_info.key)?;
    let (honeypot_key, honeypot_bump_seed) = Pubkey::find_program_address(
        &[
            b"honeypot",
            mint_info.key.as_ref(),
            &tick_size.to_le_bytes(),
            &max_amount.to_le_bytes(),
            &minimum_bank_size.to_le_bytes(),
        ],
        program_id,
    );
    let honeypot_seeds = &[
        b"honeypot",
        mint_info.key.as_ref(),
        &tick_size.to_le_bytes(),
        &max_amount.to_le_bytes(),
        &minimum_bank_size.to_le_bytes(),
        &[honeypot_bump_seed],
    ];
    let (vault_key, vault_bump_seed) = Pubkey::find_program_address(
        &[
            b"vault",
            mint_info.key.as_ref(),
            &tick_size.to_le_bytes(),
            &max_amount.to_le_bytes(),
            &minimum_bank_size.to_le_bytes(),
        ],
        program_id,
    );
    msg!("Vault {}: ", vault_key);
    let vault_seeds = &[
        b"vault",
        mint_info.key.as_ref(),
        &tick_size.to_le_bytes(),
        &max_amount.to_le_bytes(),
        &minimum_bank_size.to_le_bytes(),
        &[vault_bump_seed],
    ];
    create_or_allocate_account_raw(
        honeypot_info,
        rent_sysvar_info,
        system_program_info,
        owner_info,
        program_id,
        Honeypot::LEN as usize,
        honeypot_seeds,
    )?;
    create_or_allocate_account_raw(
        vault_info,
        rent_sysvar_info,
        system_program_info,
        owner_info,
        token_program_info.key,
        Account::LEN,
        vault_seeds,
    )?;
    invoke(
        &initialize_account(
            token_program_info.key,
            vault_info.key,
            mint_info.key,
            honeypot_info.key,
        )?,
        &[
            vault_info.clone(),
            mint_info.clone(),
            honeypot_info.clone(),
            rent_sysvar_info.clone(),
            token_program_info.clone(),
        ],
    )?;
    assert_keys_equal(honeypot_key, *honeypot_info.key)?;
    assert_keys_equal(vault_key, *vault_info.key)?;
    let mut honeypot = Honeypot::from_account_info(honeypot_info)?;
    honeypot.version = Version::HoneypotV1;
    honeypot.honeypot_bump_seed = honeypot_bump_seed;
    honeypot.vault_bump_seed = vault_bump_seed;
    honeypot.owner = *owner_info.key;
    honeypot.mint = *mint_info.key;
    honeypot.tick_size = tick_size;
    honeypot.max_amount = max_amount;
    honeypot.minimum_bank_size = minimum_bank_size;
    honeypot.serialize(&mut *honeypot_info.data.borrow_mut())?;
    Ok(())
}

fn withdraw_from_honeypot(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_to_withdraw: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let honeypot_info = next_account_info(account_info_iter)?;
    let vault_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let owner_token_account_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let honeypot = Honeypot::from_account_info(honeypot_info)?;
    assert_is_ata(owner_token_account_info, owner_info.key, mint_info.key)?;
    assert_signer(owner_info)?;
    assert_owned_by(honeypot_info, program_id)?;
    assert_owned_by(mint_info, token_program_info.key)?;
    assert_keys_equal(spl_token::id(), *token_program_info.key)?;
    assert_keys_equal(honeypot.owner, *owner_info.key)?;
    assert_keys_equal(honeypot.mint, *mint_info.key)?;
    let honeypot_seeds = &[
        b"honeypot",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.honeypot_bump_seed],
    ];
    let vault_seeds = &[
        b"vault",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.vault_bump_seed],
    ];
    let honeypot_key = Pubkey::create_program_address(honeypot_seeds, program_id).unwrap();
    let vault_key = Pubkey::create_program_address(vault_seeds, program_id).unwrap();
    assert_keys_equal(honeypot_key, *honeypot_info.key)?;
    assert_keys_equal(vault_key, *vault_info.key)?;
    invoke_signed(
        &transfer(
            token_program_info.key,
            vault_info.key,
            owner_token_account_info.key,
            honeypot_info.key,
            &[],
            amount_to_withdraw,
        )?,
        &[
            vault_info.clone(),
            owner_token_account_info.clone(),
            honeypot_info.clone(),
            token_program_info.clone(),
        ],
        &[honeypot_seeds],
    )?;
    Ok(())
}

fn initialize_guess_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let gambler_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let honeypot_info = next_account_info(account_info_iter)?;
    let vault_info = next_account_info(account_info_iter)?;
    let guess_account_info = next_account_info(account_info_iter)?;
    let rent_sysvar_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let honeypot = Honeypot::from_account_info(honeypot_info)?;
    let honeypot_seeds = &[
        b"honeypot",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.honeypot_bump_seed],
    ];
    let vault_seeds = &[
        b"vault",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.vault_bump_seed],
    ];
    let honeypot_key = Pubkey::create_program_address(honeypot_seeds, program_id).unwrap();
    let vault_key = Pubkey::create_program_address(vault_seeds, program_id).unwrap();
    assert_keys_equal(honeypot_key, *honeypot_info.key)?;
    assert_keys_equal(vault_key, *vault_info.key)?;
    let (guess_account_key, guess_account_bump_seed) = Pubkey::find_program_address(
        &[
            b"guess_account",
            gambler_info.key.as_ref(),
            vault_key.as_ref(),
        ],
        program_id,
    );
    let guess_account_seeds = &[
        b"guess_account",
        gambler_info.key.as_ref(),
        vault_key.as_ref(),
        &[guess_account_bump_seed],
    ];
    assert_keys_equal(guess_account_key, *guess_account_info.key)?;
    create_or_allocate_account_raw(
        guess_account_info,
        rent_sysvar_info,
        system_program_info,
        gambler_info,
        program_id,
        LockedGuess::LEN as usize,
        guess_account_seeds,
    )?;
    let mut locked_guesses = LockedGuess::from_account_info(guess_account_info)?;
    locked_guesses.version = Version::LockedGuessV1;
    locked_guesses.bump_seed = guess_account_bump_seed;
    locked_guesses.owner = *gambler_info.key;
    locked_guesses.vault = vault_key;
    locked_guesses.slot = 0;
    locked_guesses.active = false;
    locked_guesses.active_size = 0;
    locked_guesses.guesses = [0; 64];
    locked_guesses.serialize(&mut *guess_account_info.data.borrow_mut())?;
    Ok(())
}

fn place_guesses(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    guesses: Vec<RouletteGuess>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let gambler_info = next_account_info(account_info_iter)?;
    let gambler_token_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let honeypot_info = next_account_info(account_info_iter)?;
    let vault_info = next_account_info(account_info_iter)?;
    let guess_account_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let clock_info = next_account_info(account_info_iter)?;
    let honeypot = Honeypot::from_account_info(honeypot_info)?;
    let vault: Account = assert_initialized(vault_info)?;
    let clock = Clock::from_account_info(clock_info)?;
    let mut locked_guesses = LockedGuess::from_account_info(guess_account_info)?;
    let honeypot_seeds = &[
        b"honeypot",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.honeypot_bump_seed],
    ];
    let vault_seeds = &[
        b"vault",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.vault_bump_seed],
    ];
    let honeypot_key = Pubkey::create_program_address(honeypot_seeds, program_id).unwrap();
    let vault_key = Pubkey::create_program_address(vault_seeds, program_id).unwrap();
    assert_keys_equal(honeypot_key, *honeypot_info.key)?;
    assert_keys_equal(vault_key, *vault_info.key)?;
    assert_keys_equal(vault_key, locked_guesses.vault)?;
    assert_keys_equal(sysvar::clock::id(), *clock_info.key)?;
    assert_is_ata(gambler_token_account_info, gambler_info.key, mint_info.key)?;
    if locked_guesses.active {
        return Err(RouletteError::ActiveSpin.into());
    }
    locked_guesses.slot = clock.slot;
    locked_guesses.active = true;
    let mut total_amount: u64 = 0;
    for &guess in guesses.iter() {
        let i = guess.guess as usize;
        locked_guesses.guesses[i] = guess.amount;
        msg!("guess: {}, amount: {}", i, guess.amount);
        total_amount = total_amount
            .checked_add(guess.amount)
            .ok_or(RouletteError::NumericalOverflow)?;
    }
    let total_tokens = total_amount
        .checked_mul(honeypot.tick_size)
        .ok_or(RouletteError::NumericalOverflow)?;
    msg!(
        "total_amount: {}, total_tokens: {}, vault_amount: {}",
        total_amount,
        total_tokens,
        vault.amount,
    );
    if vault.amount <= honeypot.minimum_bank_size {
        msg!("Vault funds have been drained. The house needs to reload.");
        return Err(ProgramError::InsufficientFunds.into());
    }
    if total_tokens > honeypot.max_amount {
        msg!("Amount is too large");
        return Err(RouletteError::AmountTooLarge.into());
    }
    locked_guesses.active_size = total_tokens;
    msg!("User deposited {} tokens", total_tokens);
    invoke(
        &transfer(
            token_program_info.key,
            gambler_token_account_info.key,
            vault_info.key,
            gambler_info.key,
            &[],
            total_tokens,
        )?,
        &[
            gambler_token_account_info.clone(),
            vault_info.clone(),
            gambler_info.clone(),
            token_program_info.clone(),
        ],
    )?;
    locked_guesses.serialize(&mut *guess_account_info.data.borrow_mut())?;
    Ok(())
}

fn spin(program_id: &Pubkey, accounts: &[AccountInfo], tolerance: u64) -> ProgramResult {
    let (main_accounts, oracle_accounts) = array_refs![accounts, 9; .. ;];
    let (random_sample, slot) = random::random::sample(oracle_accounts, tolerance)?;
    let account_info_iter = &mut main_accounts.iter();
    let rng_info = next_account_info(account_info_iter)?;
    let guess_account_info = next_account_info(account_info_iter)?;
    let gambler_info = next_account_info(account_info_iter)?;
    let gambler_token_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let honeypot_info = next_account_info(account_info_iter)?;
    let vault_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let instruction_sysvar_account_info = next_account_info(account_info_iter)?;
    assert_owned_by(honeypot_info, program_id)?;
    assert_is_ata(gambler_token_account_info, gambler_info.key, mint_info.key)?;
    if *instruction_sysvar_account_info.key != sysvar::instructions::id() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let instruction_sysvar = instruction_sysvar_account_info.data.borrow();
    let current_instruction = sysvar::instructions::load_current_index(&instruction_sysvar);
    let mut idx = 0;
    let num_instructions =
        read_u16(&mut idx, &instruction_sysvar).map_err(|_| ProgramError::InvalidAccountData)?;
    msg!(
        "current_ix: {}, num_ix: {}",
        current_instruction,
        num_instructions
    );
    if current_instruction < num_instructions - 1 && num_instructions == 0 {
        msg!("This must be the only instruction in the transaction");
        return Err(RouletteError::SuspiciousTransaction.into());
    }
    let mut rng = RNG::from_account_info(rng_info)?;
    let mut locked_guesses = LockedGuess::from_account_info(guess_account_info)?;
    let honeypot = Honeypot::from_account_info(honeypot_info)?;
    let honeypot_seeds = &[
        b"honeypot",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.honeypot_bump_seed],
    ];
    let vault_seeds = &[
        b"vault",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.vault_bump_seed],
    ];
    let guess_account_seeds = &[
        b"guess_account",
        gambler_info.key.as_ref(),
        vault_info.key.as_ref(),
        &[locked_guesses.bump_seed],
    ];
    let honeypot_key = Pubkey::create_program_address(honeypot_seeds, program_id).unwrap();
    let vault_key = Pubkey::create_program_address(vault_seeds, program_id).unwrap();
    let guess_account_key =
        Pubkey::create_program_address(guess_account_seeds, program_id).unwrap();
    assert_keys_equal(honeypot_key, *honeypot_info.key)?;
    assert_keys_equal(vault_key, *vault_info.key)?;
    assert_keys_equal(vault_key, locked_guesses.vault)?;
    assert_keys_equal(guess_account_key, *guess_account_info.key)?;
    if !locked_guesses.active {
        msg!("You may only spin when the money has been committed");
        return Err(RouletteError::Inactive.into());
    }
    msg!(
        "current_slot: {}, guess_slot: {}",
        slot,
        locked_guesses.slot
    );
    if slot <= locked_guesses.slot {
        msg!("You may not spin the same slot as your guess");
        return Err(RouletteError::InvalidSlot.into());
    }
    if slot > locked_guesses.slot.checked_add(tolerance).ok_or(RouletteError::NumericalOverflow)? {
        msg!("Failure to redeem within the allotted slot time");
        return Err(RouletteError::InvalidSlot.into());
    }
    if rng.version == Version::Uninitalized {
        rng.version = Version::RNGV1;
    }
    rng.value = random_sample;
    rng.slot = slot;
    msg!("Sample {}", random_sample);
    let outcome = rng.value % 38;
    let mut reward: u64 = 0;
    let guesses = &locked_guesses.guesses;
    for i in 0..guesses.len() {
        if guesses[i] == 0 {
            continue;
        }
        let guess = RouletteGuess {
            guess: Guess::try_from_slice(&(i as u8).to_le_bytes())?,
            amount: guesses[i],
        };
        let current_payout = guess.get_payout(outcome);
        reward = reward
            .checked_add(current_payout)
            .ok_or(ProgramError::InvalidInstructionData)?;
        msg!(
            "payout: {}, guess: {}, size: {}",
            current_payout,
            i,
            guesses[i]
        );
    }
    let total_reward = reward
        .checked_mul(honeypot.tick_size)
        .ok_or(RouletteError::NumericalOverflow)?;
    msg!("outcome: {}, total_reward: {}", outcome, total_reward);
    if total_reward > 0 {
        msg!("User won {} tokens", total_reward);
        invoke_signed(
            &transfer(
                token_program_info.key,
                vault_info.key,
                gambler_token_account_info.key,
                honeypot_info.key,
                &[],
                total_reward,
            )?,
            &[
                vault_info.clone(),
                gambler_token_account_info.clone(),
                honeypot_info.clone(),
                token_program_info.clone(),
            ],
            &[honeypot_seeds],
        )?;
    }
    locked_guesses.active = false;
    locked_guesses.active_size = 0;
    locked_guesses.guesses = [0; 64];
    rng.serialize(&mut *rng_info.data.borrow_mut())?;
    locked_guesses.serialize(&mut *guess_account_info.data.borrow_mut())?;
    Ok(())
}

fn try_cancel(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let guess_account_info = next_account_info(account_info_iter)?;
    let gambler_info = next_account_info(account_info_iter)?;
    let gambler_token_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let honeypot_info = next_account_info(account_info_iter)?;
    let vault_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let mut locked_guesses = LockedGuess::from_account_info(guess_account_info)?;
    let honeypot = Honeypot::from_account_info(honeypot_info)?;
    if !locked_guesses.active {
        return Ok(());
    }
    assert_owned_by(honeypot_info, program_id)?;
    assert_is_ata(gambler_token_account_info, gambler_info.key, mint_info.key)?;
    let honeypot_seeds = &[
        b"honeypot",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.honeypot_bump_seed],
    ];
    let vault_seeds = &[
        b"vault",
        mint_info.key.as_ref(),
        &honeypot.tick_size.to_le_bytes(),
        &honeypot.max_amount.to_le_bytes(),
        &honeypot.minimum_bank_size.to_le_bytes(),
        &[honeypot.vault_bump_seed],
    ];
    let guess_account_seeds = &[
        b"guess_account",
        gambler_info.key.as_ref(),
        vault_info.key.as_ref(),
        &[locked_guesses.bump_seed],
    ];
    let honeypot_key = Pubkey::create_program_address(honeypot_seeds, program_id).unwrap();
    let vault_key = Pubkey::create_program_address(vault_seeds, program_id).unwrap();
    let guess_account_key =
        Pubkey::create_program_address(guess_account_seeds, program_id).unwrap();
    assert_keys_equal(honeypot_key, *honeypot_info.key)?;
    assert_keys_equal(vault_key, *vault_info.key)?;
    assert_keys_equal(vault_key, locked_guesses.vault)?;
    assert_keys_equal(guess_account_key, *guess_account_info.key)?;
    msg!("Refunding {} tokens to user", locked_guesses.active_size);
    invoke_signed(
        &transfer(
            token_program_info.key,
            vault_info.key,
            gambler_token_account_info.key,
            honeypot_info.key,
            &[],
            locked_guesses.active_size,
        )?,
        &[
            vault_info.clone(),
            gambler_token_account_info.clone(),
            honeypot_info.clone(),
            token_program_info.clone(),
        ],
        &[honeypot_seeds],
    )?;
    locked_guesses.active = false;
    locked_guesses.active_size = 0;
    locked_guesses.guesses = [0; 64];
    locked_guesses.serialize(&mut *guess_account_info.data.borrow_mut())?;
    Ok(())
}
