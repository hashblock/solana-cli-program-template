//! instruction Contains the main ProgramInstruction enum

use crate::error::SampleError;
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
/// All custom program instructions
pub enum ProgramInstruction {
    InitializeAccount,
    MintToAccount { key: String, value: String },
    TransferBetweenAccounts { key: String },
    BurnFromAccount { key: String },
}

impl ProgramInstruction {
    /// Unpack inbound buffer to associated Instruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let block = Vec::<Vec<u8>>::try_from_slice(input).unwrap();
        msg!("Instruction data {:?}", block);
        let header = &block[0];
        msg!("Header {:?}", header);
        let instruction = header[0];
        msg!("Instruction {:?}", instruction);
        match instruction {
            0 => Ok(ProgramInstruction::InitializeAccount),
            1 => Ok(Self::MintToAccount {
                key: String::try_from_slice(&block[1]).unwrap(),
                value: String::try_from_slice(&block[2]).unwrap(),
            }),
            2 => Ok(Self::TransferBetweenAccounts {
                key: String::try_from_slice(&block[1]).unwrap(),
            }),
            3 => Ok(Self::BurnFromAccount {
                key: String::try_from_slice(&block[1]).unwrap(),
            }),
            _ => Err(SampleError::DeserializationFailure.into()),
        }
        // match String::try_from_slice(&block[1]) {
        //     Ok(key) => {
        //         Ok(match instruction {
        //             0 => ProgramInstruction::InitializeAccount,
        //             // Mint expects two strings (key, value)
        //             1 => Self::MintToAccount {
        //                 key,
        //                 value: String::try_from_slice(&block[2]).unwrap(),
        //             },
        //             2 => Self::TransferBetweenAccounts { key },
        //             3 => Self::BurnFromAccount { key },
        //             _ => {
        //                 msg!("Attempting unknown instruction {}", instruction);
        //                 unreachable!()
        //             }
        //         })
        //     }
        //     Err(_) => Err(SampleError::DeserializationFailure.into()),
        // }
    }
}
