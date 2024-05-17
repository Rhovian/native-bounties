use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    },
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoidhInstruction {
    /// Create a new bounty
    ///
    /// Accounts expected by this instruction:
    ///   1. `[writeable, signer]` Funding account (must be a system account)
    ///   2. `[writeable]` Bounty to be created (program-derived address)
    ///   3. `[signer]` mint (Unique identifier for the bounty)
    ///   4. `[]` payment_mint
    ///   5. `[writeable]` Funding account token account for payment mint
    ///   6. `[writeable]` Bounty account token account for payment mint
    ///   7. `[]` Token program
    ///   8. `[]` Associated token program
    ///   9. `[]` System program
    CreateBounty {
        /// SHA256 of the (HASH_PREFIX + Name) of the record to create, hashing
        /// is done off-chain
        hashed_name: Vec<u8>,

        /// Number of lamports to fund the name record with
        lamports: u64,

        /// Number of bytes of memory to allocate in addition to the
        space: u32,


    },
}