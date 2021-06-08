//! instruction Contains the main ProgramInstruction enum

use borsh::BorshDeserialize;
use crate::error::SampleError;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
/// All custom program instructions
pub enum ProgramInstruction {
    InitializeAccount,
    MintToAccount {
        key: String,
        value: String,
    },
    TransferBetweenAccounts {
        key: String,
    },
    BurnFromAccount {
        key: String,
    },
}

impl ProgramInstruction {
    /// Unpack inbound buffer to associated Instruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let block = Vec::<Vec<u8>>::try_from_slice(input).unwrap();
        let header = &block[0];
        let instruction =  header[0];
        match String::try_from_slice(&block[1]) {
            Ok(key) => {
                Ok(match instruction {
                    0 => ProgramInstruction::InitializeAccount,
                    // Mint expects two strings (key, value)
                    1 => Self::MintToAccount {
                        key,
                        value: String::try_from_slice(&block[2]).unwrap(),
                    },
                    2 => Self::TransferBetweenAccounts {
                        key,
                    },
                    3 => Self::BurnFromAccount {
                        key,
                    },
                    _ => {
                        msg!("Attempting unknown instruction {}", instruction);
                        unreachable!()
                    }
                })
            },
            Err(_) => Err(SampleError::DeserializationFailure.into())
        }
    }
}
