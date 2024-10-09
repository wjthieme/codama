/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  AccountRole,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getUtf8Decoder,
  getUtf8Encoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type TransactionSigner,
} from '@solana/web3.js';
import { MEMO_PROGRAM_ADDRESS } from '../programs';

export type AddMemoInstruction<
  TProgram extends string = typeof MEMO_PROGRAM_ADDRESS,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<TRemainingAccounts>;

export type AddMemoInstructionData = { memo: string };

export type AddMemoInstructionDataArgs = AddMemoInstructionData;

export function getAddMemoInstructionDataEncoder(): Encoder<AddMemoInstructionDataArgs> {
  return getStructEncoder([['memo', getUtf8Encoder()]]);
}

export function getAddMemoInstructionDataDecoder(): Decoder<AddMemoInstructionData> {
  return getStructDecoder([['memo', getUtf8Decoder()]]);
}

export function getAddMemoInstructionDataCodec(): Codec<
  AddMemoInstructionDataArgs,
  AddMemoInstructionData
> {
  return combineCodec(
    getAddMemoInstructionDataEncoder(),
    getAddMemoInstructionDataDecoder()
  );
}

export type AddMemoInput = {
  memo: AddMemoInstructionDataArgs['memo'];
  signers?: Array<TransactionSigner>;
};

export function getAddMemoInstruction<
  TProgramAddress extends Address = typeof MEMO_PROGRAM_ADDRESS,
>(
  input: AddMemoInput,
  config?: { programAddress?: TProgramAddress }
): AddMemoInstruction<TProgramAddress> {
  // Program address.
  const programAddress = config?.programAddress ?? MEMO_PROGRAM_ADDRESS;

  // Original args.
  const args = { ...input };

  // Remaining accounts.
  const remainingAccounts: IAccountMeta[] = (args.signers ?? []).map(
    (signer) => ({
      address: signer.address,
      role: AccountRole.READONLY_SIGNER,
      signer,
    })
  );

  const instruction = {
    accounts: remainingAccounts,
    programAddress,
    data: getAddMemoInstructionDataEncoder().encode(
      args as AddMemoInstructionDataArgs
    ),
  } as AddMemoInstruction<TProgramAddress>;

  return instruction;
}

export type ParsedAddMemoInstruction<
  TProgram extends string = typeof MEMO_PROGRAM_ADDRESS,
> = {
  programAddress: Address<TProgram>;
  data: AddMemoInstructionData;
};

export function parseAddMemoInstruction<TProgram extends string>(
  instruction: IInstruction<TProgram> & IInstructionWithData<Uint8Array>
): ParsedAddMemoInstruction<TProgram> {
  return {
    programAddress: instruction.programAddress,
    data: getAddMemoInstructionDataDecoder().decode(instruction.data),
  };
}
