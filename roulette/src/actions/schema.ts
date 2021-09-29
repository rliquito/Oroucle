import {
  RNG,
  RouletteGuess,
  LockedGuess,
  InitializeArgs,
  SampleArgs,
  InitializeHoneypotArgs,
  WithdrawFromHoneypotArgs,
  InitializeGuessAccountArgs,
  PlaceGuessesArgs,
  SpinArgs,
  TryCancelArgs,
  Honeypot,
} from "./state";

export const schema = new Map<any, any>([
  [
    InitializeArgs,
    {
      kind: "struct",
      fields: [["instruction", "u8"]],
    },
  ],
  [
    SampleArgs,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
        ["tolerance", "u64"],
      ],
    },
  ],
  [
    InitializeHoneypotArgs,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
        ["tickSize", "u64"],
        ["maxBetSize", "u64"],
        ["minimumBankSize", "u64"],
      ],
    },
  ],
  [
    WithdrawFromHoneypotArgs,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
        ["amountToWithdraw", "u64"],
      ],
    },
  ],
  [
    InitializeGuessAccountArgs,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
      ],
    },
  ],
  [
    PlaceGuessesArgs,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
        ["guesses", [RouletteGuess]],
      ],
    },
  ],
  [
    SpinArgs,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
        ["tolerance", "u64"],
      ],
    },
  ],
  [
    TryCancelArgs,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
      ],
    },
  ],
  [
    Honeypot,
    {
      kind: "struct",
      fields: [
        ["version", "u8"],
        ["honeypotBumpSeed", "u8"],
        ["vaultBumpSeed", "u8"],
        ["owner", "pubkeyAsString"],
        ["mint", "pubkeyAsString"],
        ["tickSize", "u64"],
        ["maxAmount", "u64"],
        ["minimumBankSize", "u64"],
        ["owedAmount", "u64"],
      ],
    },
  ],
  [
    RouletteGuess,
    {
      kind: "struct",
      fields: [
        ["guess", "u8"],
        ["amount", "u64"],
      ],
    },
  ],
  [
    RNG,
    {
      kind: "struct",
      fields: [
        ["version", "u8"],
        ["sample", "u64"],
        ["slot", "u64"],
      ],
    },
  ],
]);
