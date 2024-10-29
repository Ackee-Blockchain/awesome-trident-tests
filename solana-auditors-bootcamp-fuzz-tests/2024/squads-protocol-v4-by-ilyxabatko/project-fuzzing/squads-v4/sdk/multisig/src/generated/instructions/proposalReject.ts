/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  ProposalVoteArgs,
  proposalVoteArgsBeet,
} from '../types/ProposalVoteArgs'

/**
 * @category Instructions
 * @category ProposalReject
 * @category generated
 */
export type ProposalRejectInstructionArgs = {
  args: ProposalVoteArgs
}
/**
 * @category Instructions
 * @category ProposalReject
 * @category generated
 */
export const proposalRejectStruct = new beet.FixableBeetArgsStruct<
  ProposalRejectInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['args', proposalVoteArgsBeet],
  ],
  'ProposalRejectInstructionArgs'
)
/**
 * Accounts required by the _proposalReject_ instruction
 *
 * @property [] multisig
 * @property [_writable_, **signer**] member
 * @property [_writable_] proposal
 * @category Instructions
 * @category ProposalReject
 * @category generated
 */
export type ProposalRejectInstructionAccounts = {
  multisig: web3.PublicKey
  member: web3.PublicKey
  proposal: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const proposalRejectInstructionDiscriminator = [
  243, 62, 134, 156, 230, 106, 246, 135,
]

/**
 * Creates a _ProposalReject_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category ProposalReject
 * @category generated
 */
export function createProposalRejectInstruction(
  accounts: ProposalRejectInstructionAccounts,
  args: ProposalRejectInstructionArgs,
  programId = new web3.PublicKey('SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf')
) {
  const [data] = proposalRejectStruct.serialize({
    instructionDiscriminator: proposalRejectInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.multisig,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.member,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.proposal,
      isWritable: true,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}