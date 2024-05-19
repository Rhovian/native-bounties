use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PoidhError {
    /// 0 Failed to unpack instruction data
    #[error("InstructionUnpackError")]
    InstructionUnpackError,
    /// 1 BountyNotInitialized
    #[error("BountyNotInitialized")]
    BountyNotInitialized,
    /// 2 InvalidOwner
    #[error("InvalidOwner")]
    InvalidOwner,
    /// 3 BountyAlreadyClaimed
    #[error("BountyAlreadyClaimed")]
    BountyAlreadyClaimed,
}

impl PrintProgramError for PoidhError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<PoidhError> for ProgramError {
    fn from(e: PoidhError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PoidhError {
    fn type_of() -> &'static str {
        "Metadata Error"
    }
}
