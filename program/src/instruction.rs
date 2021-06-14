//! instruction Contains the main ProgramInstruction enum

use {
    crate::error::SampleError, borsh::BorshDeserialize, solana_program::program_error::ProgramError,
};

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
        let instruction = &block[0][0];
        // let header = &block[0];
        // let instruction = header[0];
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
    }
}
