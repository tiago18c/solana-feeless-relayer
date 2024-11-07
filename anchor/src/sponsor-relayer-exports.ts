// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { PublicKey } from '@solana/web3.js'
import SponsorRelayerIDL from '../target/idl/sponsor_relayer.json'
import type { SponsorRelayer } from '../target/types/sponsor_relayer'

// Re-export the generated IDL and type
export { SponsorRelayer, SponsorRelayerIDL }

// The programId is imported from the program IDL.
export const SPONSOR_RELAYER_PROGRAM_ID = new PublicKey(SponsorRelayerIDL.address)

// This is a helper function to get the Basic Anchor program.
export function getSponsorRelayerProgram(provider: AnchorProvider) {
  return new Program(SponsorRelayerIDL as SponsorRelayer, provider)
}

export function getSponsorshipAccountPda(sponsor: PublicKey) {
  return PublicKey.findProgramAddressSync([Buffer.from("sponsorship"), sponsor.toBuffer()], SPONSOR_RELAYER_PROGRAM_ID);
}

export function getRelayerAccountPda(sponsorship: PublicKey, authority: PublicKey) {
  return PublicKey.findProgramAddressSync([Buffer.from("relayer"), sponsorship.toBuffer(), authority.toBuffer()], SPONSOR_RELAYER_PROGRAM_ID);
}
