use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        msg,
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

impl Bounty {
    pub const MAX_NAME_LENGTH: usize = 32;
    pub const MAX_DESCRIPTION_LENGTH: usize = 256;

    pub const LEN: usize = 32
        + 32
        + 32
        + (4 + Self::MAX_NAME_LENGTH)
        + (4 + Self::MAX_DESCRIPTION_LENGTH)
        + 8
        + 32
        + 8
        + 8
        + 1;

    pub fn save(&self, data: &mut [u8]) -> Result<(), ProgramError> {
        let mut bytes = Vec::with_capacity(Self::LEN);
        borsh::to_writer(&mut bytes, self)?;
        data[..bytes.len()].copy_from_slice(&bytes);
        Ok(())
    }
}

impl Pack for Bounty {
    const LEN: usize = Bounty::LEN;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut p = src;
        Bounty::deserialize(&mut p).map_err(|_| {
            msg!("Failed to deserialize name record");
            ProgramError::InvalidAccountData
        })
    }
}
