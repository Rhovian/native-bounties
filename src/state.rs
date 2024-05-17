use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::AccountInfo,
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack, Sealed},
        pubkey::Pubkey,
    },
};

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Default, PartialEq)]
pub struct Bounty {
    pub owner: Pubkey,
    pub mint: Pubkey, // is this needed?
    pub payment_mint: Pubkey,
    pub name: String,
    pub description: String,
    pub amount: u64,
    pub claimer: Pubkey,
    pub created_at: u64,
    pub claim_id: u64,
    pub is_initialized: bool,
}

impl IsInitialized for Bounty {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Sealed for Bounty {} // what does this do?

impl Pack for Bounty {
    const LEN: usize = 8 + 32 + 32 + 32 + 32 + 8 + 1 + 8 + 8 + 1;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        Bounty::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)
    }
}

pub fn write_data(account: &AccountInfo, input: &[u8], offset: usize) {
    let mut account_data = account.data.borrow_mut();
    account_data[offset..offset.saturating_add(input.len())].copy_from_slice(input);
}
