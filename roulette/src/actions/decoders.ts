import { deserializeUnchecked } from 'borsh';
import {
  Honeypot,
  RNG,
  RouletteGuess,
} from './state'

import { schema } from './schema'

export const decodeRNG = (buffer: Buffer) => {
  return deserializeUnchecked(schema, RNG, buffer) as RNG;
};

export const decodeHoneypot = (buffer: Buffer) => {
  return deserializeUnchecked(schema, Honeypot, buffer) as Honeypot;
};

export const decodeRouletteGuess = (buffer: Buffer) => {
  return deserializeUnchecked(schema, RouletteGuess, buffer) as RouletteGuess;
};