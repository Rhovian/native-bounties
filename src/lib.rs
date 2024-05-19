#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

solana_program::declare_id!("mataqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
