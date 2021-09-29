use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Copy)]
pub enum Version {
    Uninitalized,
    Tombstone,
    RNGV1,
    HoneypotV1,
    LockedGuessV1,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct RNG {
    pub version: Version,
    pub value: u64,
    pub slot: u64,
}

impl RNG {
    pub const LEN: i64 = 1 + 8 + 8;

    pub fn from_account_info(a: &AccountInfo) -> Result<RNG, ProgramError> {
        let rng = RNG::try_from_slice(&a.data.borrow())?;
        Ok(rng)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Honeypot {
    pub version: Version,
    pub honeypot_bump_seed: u8,
    pub vault_bump_seed: u8,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub tick_size: u64,
    pub max_amount: u64,
    pub minimum_bank_size: u64,
}

impl Honeypot {
    pub const LEN: i64 = 1 + 1 + 1 + 32 + 32 + 8 + 8 + 8;

    pub fn from_account_info(a: &AccountInfo) -> Result<Honeypot, ProgramError> {
        let hp = Honeypot::try_from_slice(&a.data.borrow())?;
        Ok(hp)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct LockedGuess {
    pub version: Version,
    pub bump_seed: u8,
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub slot: u64,
    pub active: bool,
    pub active_size: u64,
    pub guesses: [u64; 64],
}

impl LockedGuess {
    pub const LEN: i64 = 1 + 1 + 32 + 32 + 8 + 1 + 8 + 8 * 64;

    pub fn from_account_info(a: &AccountInfo) -> Result<LockedGuess, ProgramError> {
        let lb = LockedGuess::try_from_slice(&a.data.borrow())?;
        Ok(lb)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Copy)]
pub enum Guess {
    Zero = 0,
    DoubleZero = 1,
    R1 = 2,
    B2 = 3,
    R3 = 4,
    B4 = 5,
    R5 = 6,
    B6 = 7,
    R7 = 8,
    B8 = 9,
    R9 = 10,
    B10 = 11,
    B11 = 12,
    R12 = 13,
    B13 = 14,
    R14 = 15,
    B15 = 16,
    R16 = 17,
    B17 = 18,
    R18 = 19,
    R19 = 20,
    B20 = 21,
    R21 = 22,
    B22 = 23,
    R23 = 24,
    B24 = 25,
    R25 = 26,
    B26 = 27,
    R27 = 28,
    B28 = 29,
    B29 = 30,
    R30 = 31,
    B31 = 32,
    R32 = 33,
    B33 = 34,
    R34 = 35,
    B35 = 36,
    R36 = 37,
    Red = 38,
    Black = 39,
    Even = 40,
    Odd = 41,
    Col1 = 42,
    Col2 = 43,
    Col3 = 44,
    Dozen1 = 45,
    Dozen2 = 46,
    Dozen3 = 47,
    Low = 48,
    High = 49,
}

pub fn is_red(number: u64) -> bool {
    let red_numbers: Vec<u64> = vec![
        1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
    ];
    red_numbers.contains(&number)
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Copy)]
pub struct RouletteGuess {
    pub guess: Guess,
    pub amount: u64,
}

impl RouletteGuess {
    pub fn get_payout(&self, outcome: u64) -> u64 {
        match self.guess {
            Guess::Zero => {
                if outcome == 0 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::DoubleZero => {
                if outcome == 37 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R1 => {
                if outcome == 1 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B2 => {
                if outcome == 2 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R3 => {
                if outcome == 3 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B4 => {
                if outcome == 4 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R5 => {
                if outcome == 5 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B6 => {
                if outcome == 6 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R7 => {
                if outcome == 7 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B8 => {
                if outcome == 8 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R9 => {
                if outcome == 9 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B10 => {
                if outcome == 10 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B11 => {
                if outcome == 11 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R12 => {
                if outcome == 12 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B13 => {
                if outcome == 13 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R14 => {
                if outcome == 14 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B15 => {
                if outcome == 15 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R16 => {
                if outcome == 16 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B17 => {
                if outcome == 17 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R18 => {
                if outcome == 18 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R19 => {
                if outcome == 19 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B20 => {
                if outcome == 20 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R21 => {
                if outcome == 21 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B22 => {
                if outcome == 22 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R23 => {
                if outcome == 23 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B24 => {
                if outcome == 24 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R25 => {
                if outcome == 25 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B26 => {
                if outcome == 26 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R27 => {
                if outcome == 27 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B28 => {
                if outcome == 28 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B29 => {
                if outcome == 29 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R30 => {
                if outcome == 30 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B31 => {
                if outcome == 31 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R32 => {
                if outcome == 32 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B33 => {
                if outcome == 33 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R34 => {
                if outcome == 34 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::B35 => {
                if outcome == 35 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::R36 => {
                if outcome == 36 {
                    self.amount * 36
                } else {
                    0
                }
            }
            Guess::Red => {
                if outcome != 0 && outcome != 37 && is_red(outcome) {
                    self.amount * 2
                } else {
                    0
                }
            }
            Guess::Black => {
                if outcome != 0 && outcome != 37 && !is_red(outcome) {
                    self.amount * 2
                } else {
                    0
                }
            }
            Guess::Even => {
                if outcome != 0 && outcome % 2 == 0 {
                    self.amount * 2
                } else {
                    0
                }
            }
            Guess::Odd => {
                if outcome != 37 && outcome % 2 == 1 {
                    self.amount * 2
                } else {
                    0
                }
            }
            Guess::Col1 => {
                if outcome != 37 && outcome % 3 == 1 {
                    self.amount * 3
                } else {
                    0
                }
            }
            Guess::Col2 => {
                if outcome % 3 == 2 {
                    self.amount * 3
                } else {
                    0
                }
            }
            Guess::Col3 => {
                if outcome != 0 && outcome % 3 == 0 {
                    self.amount * 3
                } else {
                    0
                }
            }
            Guess::Dozen1 => {
                if outcome > 0 && outcome <= 12 {
                    self.amount * 3
                } else {
                    0
                }
            }
            Guess::Dozen2 => {
                if outcome > 12 && outcome <= 24 {
                    self.amount * 3
                } else {
                    0
                }
            }
            Guess::Dozen3 => {
                if outcome > 24 && outcome < 37 {
                    self.amount * 3
                } else {
                    0
                }
            }
            Guess::Low => {
                if outcome > 0 && outcome <= 18 {
                    self.amount * 2
                } else {
                    0
                }
            }
            Guess::High => {
                if outcome > 18 && outcome < 37 {
                    self.amount * 2
                } else {
                    0
                }
            }
        }
    }
}
