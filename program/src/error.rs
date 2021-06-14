//! error are specific errors in our custom program

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    program_error::{PrintProgramError, ProgramError},
};
use std::fmt;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SampleError {
    InvalidInstruction,
    DeserializationFailure,
    AlreadyInitializedState,
    KeyNotFoundInAccount,
    KeyAlreadyExists,
    InsufficientFundsForTransaction,
    UnknownError,
}

impl From<SampleError> for ProgramError {
    fn from(e: SampleError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SampleError {
    fn type_of() -> &'static str {
        "SampleError"
    }
}

impl fmt::Display for SampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SampleError::InvalidInstruction => f.write_str("Invalid instruction"),
            SampleError::DeserializationFailure => f.write_str("Error Deserializing input data"),
            SampleError::AlreadyInitializedState => f.write_str("Account already initialized"),
            SampleError::KeyNotFoundInAccount => f.write_str("Account does not contain key"),
            SampleError::KeyAlreadyExists => f.write_str("Account already contains key"),
            SampleError::UnknownError => f.write_str("Unknown error condiiton"),
            SampleError::InsufficientFundsForTransaction => {
                f.write_str("Not enough funds to process transaction")
            }
        }
    }
}

impl PrintProgramError for SampleError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            SampleError::InvalidInstruction => println!("Error: Invalid instruction"),
            SampleError::DeserializationFailure => println!("Error Deserializing input data"),
            SampleError::AlreadyInitializedState => println!("Account already initialized"),
            SampleError::KeyNotFoundInAccount => println!("Account does not contain key"),
            SampleError::KeyAlreadyExists => println!("Account already contains key"),
            SampleError::UnknownError => println!("Unknown error condiiton"),
            SampleError::InsufficientFundsForTransaction => {
                println!("Not enough funds to process transaction")
            }
        }
    }
}
