import { StringPublicKey } from '../utils';
import BN from 'bn.js'

export class RNG {
  version: number;
  sample: BN;
  slot: BN;
  constructor(args: {
    version: number;
    sample: BN;
    slot: BN;
  }) {
    this.version = args.version;
    this.sample = args.sample;
    this.slot = args.slot;
  }
}

export class Honeypot {
  version: number;
  honeypotBumpSeed: number;
  vaultBumpSeed: number;
  owner: StringPublicKey;
  mint: StringPublicKey;
  tickSize: BN;
  maxAmount: BN;
  minimumBankSize: BN;
  owedAmount: BN;
  constructor(args: {
    version: number;
    honeypotBumpSeed: number;
    vaultBumpSeed: number;
    owner: StringPublicKey;
    mint: StringPublicKey;
    tickSize: BN;
    maxAmount: BN;
    minimumBankSize: BN;
    owedAmount: BN;
  }) {
    this.version = args.version;
    this.honeypotBumpSeed = args.honeypotBumpSeed;
    this.vaultBumpSeed = args.vaultBumpSeed;
    this.owner = args.owner;
    this.mint = args.mint;
    this.tickSize = args.tickSize;
    this.maxAmount = args.maxAmount;
    this.minimumBankSize = args.minimumBankSize;
    this.owedAmount = args.owedAmount;
  }
}

export class LockedGuess {
  version: number;
  bumpSeed: number;
  owner: StringPublicKey;
  vault: StringPublicKey;
  slot: BN; 
  reward: BN; 
  active: number; 
  guesses: BN[]; 
  constructor(args: {
    version: number;
    bumpSeed: number;
    owner: StringPublicKey;
    vault: StringPublicKey;
    slot: BN; 
    reward: BN; 
    active: number; 
    guesses: BN[]; 
  }) {
    this.version = args.version;
    this.bumpSeed = args.bumpSeed;
    this.owner = args.owner;
    this.vault = args.vault;
    this.slot = args.slot;
    this.reward = args.reward;
    this.active = args.active;
    this.guesses = args.guesses;
  }
}

export class RouletteGuess {
  guess: number;
  amount: BN;
  constructor(args: {
    guess: number;
    amount: BN;
  }) {
    this.guess = args.guess;
    this.amount = args.amount;
  }
}

export class InitializeArgs {
  instruction: number = 0;
}

export class SampleArgs {
  instruction: number = 1;
  tolerance: BN;
  constructor(args: {
      tolerance: BN
  }) {
    this.tolerance = args.tolerance;
  }
}

export class InitializeHoneypotArgs {
  instruction: number = 2;
  tickSize: BN;
  maxBetSize: BN;
  minimumBankSize: BN;
  constructor(args: {
    tickSize: BN,
    maxBetSize: BN;
    minimumBankSize: BN;
  }) {
    this.tickSize = args.tickSize;
    this.maxBetSize = args.maxBetSize;
    this.minimumBankSize = args.minimumBankSize;
  }
}

export class WithdrawFromHoneypotArgs {
  instruction: number = 3;
  amountToWithdraw: BN;
  constructor(args: {
    amountToWithdraw: BN;
  }) {
    this.amountToWithdraw = args.amountToWithdraw;
  }
}

export class InitializeGuessAccountArgs {
  instruction: number = 4;
}

export class PlaceGuessesArgs {
  instruction: number = 5;
  guesses: RouletteGuess[];
  constructor(args: {
    guesses: RouletteGuess[];
  }) {
    this.guesses = args.guesses;
  }
}

export class SpinArgs {
  instruction: number = 6;
  tolerance: BN;
  constructor(args: {
    tolerance: BN;
  }) {
    this.tolerance = args.tolerance;
  }
}

export class TryCancelArgs {
  instruction: number = 7;
}