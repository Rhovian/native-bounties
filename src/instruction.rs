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
    /// 7. `[]` Token program
    /// 8. `[]` Associated token program
    /// 9. `[]` System program
    CreateBounty(CreateBountyArgs),
}
