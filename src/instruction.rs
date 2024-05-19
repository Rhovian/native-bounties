use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateBountyArgs {
    pub name: String,
    pub description: String,
    pub amount: u64,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PoidhInstruction {
    /// Create a new bounty
    ///
    /// Accounts expected by this instruction:
    /// 1. `[writeable, signer]` Funding account (must be a system account)
    /// 2. `[writeable]` Bounty to be created (program-derived address)
    /// 3. `[signer]` mint (Unique identifier for the bounty)
    /// 4. `[]` System program
    CreateBounty(CreateBountyArgs),
    /// Close a bounty
    ///
    /// Accounts expected by this instruction:
    /// 1. `[writeable, signer]` Owner of the bounty
    /// 2. `[writeable]` Bounty account
    /// 3. `[writeable]` Mint account
    /// 4. `[]` System program
    CloseBounty,
}
