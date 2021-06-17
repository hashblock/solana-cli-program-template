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
    MintToAccountWithFee { key: String, value: String },
    TransferBetweenAccountsWithFee { key: String },
    BurnFromAccountWithFee { key: String },
}

impl ProgramInstruction {
    /// Unpack inbound buffer to associated Instruction
    /// The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let block = Vec::<Vec<u8>>::try_from_slice(input).unwrap();
        match block[0][0] {
            0 => Ok(ProgramInstruction::InitializeAccount),
            1 => Ok(Self::MintToAccount {
                key: String::try_from_slice(&block[1])?,
                value: String::try_from_slice(&block[2])?,
            }),
            2 => Ok(Self::TransferBetweenAccounts {
                key: String::try_from_slice(&block[1])?,
            }),
            3 => Ok(Self::BurnFromAccount {
                key: String::try_from_slice(&block[1])?,
            }),
            4 => Ok(Self::MintToAccountWithFee {
                key: String::try_from_slice(&block[1])?,
                value: String::try_from_slice(&block[2])?,
            }),
            5 => Ok(Self::TransferBetweenAccountsWithFee {
                key: String::try_from_slice(&block[1])?,
            }),
            6 => Ok(Self::BurnFromAccountWithFee {
                key: String::try_from_slice(&block[1])?,
            }),
            _ => Err(SampleError::DeserializationFailure.into()),
        }
    }
}
