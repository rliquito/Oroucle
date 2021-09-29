use crate::state::RouletteGuess;
use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct SampleArgs {
    pub tolerance: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitializeHoneypotArgs {
    pub tick_size: u64,
    pub max_amount: u64,
    pub minimum_bank_size: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct WithdrawFromHoneypotArgs {
    pub amount_to_withdraw: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct PlaceGuessArgs {
    pub guesses: Vec<RouletteGuess>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct SpinArgs {
    pub tolerance: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum RandomInstruction {
    Initialize,
    Sample(SampleArgs),
    InitializeHoneypot(InitializeHoneypotArgs),
    WithdrawFromHoneypot(WithdrawFromHoneypotArgs),
    InitializeGuessAccount,
    PlaceGuesses(PlaceGuessArgs),
    Spin(SpinArgs),
    TryCancel,
}
