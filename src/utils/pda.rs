use solana_program::pubkey::Pubkey;

pub const BOUNTY: &str = "bounty";

pub fn find_bounty_account(owner: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[BOUNTY.as_bytes(), owner.as_ref(), mint.as_ref()],
        &crate::ID,
    )
}
