//! processor conducts instruction execution

use crate::{
    account_state::ProgramAccountState, error::SampleError, instruction::ProgramInstruction,
};

use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};
pub struct Processor {}

impl Processor {
    /// Checks each tracking account to confirm it is owned by our program
    /// This function assumes that the program account is always the last
    /// in the array
    fn check_account_ownership(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        // Accounts must be owned by the program.
        for account in accounts.iter().take(accounts.len() - 1) {
            if account.owner != program_id {
                msg!(
                    "Fail: The tracking account owner is {} and it should be {}.",
                    account.owner,
                    program_id
                );
                return Err(ProgramError::IncorrectProgramId);
            }
        }
        Ok(())
    }

    /// Initialize the programs account, which is the first in accounts
    fn initialize_account(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_account = next_account_info(account_info_iter)?;
        let mut account_data = program_account.data.borrow_mut();
        let account_state = ProgramAccountState::unpack_unchecked(&account_data)?;

        if account_state.is_initialized() {
            return Err(SampleError::AlreadyInitializedState.into());
        }
        let mut default_account_state = ProgramAccountState::default();
        default_account_state.set_initialized();

        ProgramAccountState::pack(default_account_state, &mut account_data).unwrap();
        Ok(())
    }
    /// Mint a key/pair to the programs account, which is the first in accounts
    fn mint_keypair_to_account(
        accounts: &[AccountInfo],
        key: String,
        value: String,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_account = next_account_info(account_info_iter)?;
        let mut account_data = program_account.data.borrow_mut();
        let mut account_state = ProgramAccountState::unpack_unchecked(&account_data)?;
        account_state.add(key, value)?;
        ProgramAccountState::pack(account_state, &mut account_data)?;
        Ok(())
    }
    /// Transfer a key/pair from one program account to another
    /// "from" account is first and "to" account is second  in accounts
    fn transfer_keypair_to_account(accounts: &[AccountInfo], key: String) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        // Transfer from this account
        let from_program_account = next_account_info(account_info_iter)?;
        let mut from_account_data = from_program_account.data.borrow_mut();
        let mut from_account_state = ProgramAccountState::unpack_unchecked(&from_account_data)?;
        // To this account
        let to_program_account = next_account_info(account_info_iter)?;
        let mut to_account_data = to_program_account.data.borrow_mut();
        let mut to_account_state = ProgramAccountState::unpack_unchecked(&to_account_data)?;
        // Transfer the goods
        match from_account_state.remove(&key) {
            Ok(value) => {
                to_account_state.add(key, value)?;
                ProgramAccountState::pack(from_account_state, &mut from_account_data)?;
                ProgramAccountState::pack(to_account_state, &mut to_account_data)?;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
    /// Burn a key/pair from the programs account, which is the first in accounts
    fn burn_keypair_from_account(accounts: &[AccountInfo], key: String) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_account = next_account_info(account_info_iter)?;
        let mut account_data = program_account.data.borrow_mut();
        let mut account_state = ProgramAccountState::unpack_unchecked(&account_data)?;
        match account_state.remove(&key) {
            Ok(_) => {
                ProgramAccountState::pack(account_state, &mut account_data)?;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Main processing entry point dispatches to specific
    /// instruction handlers
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Received process request");
        // Check the account for program relationship
        if let Err(error) = Self::check_account_ownership(program_id, accounts) {
            return Err(error);
        }
        // Unpack the inbound data, mapping instruction to appropriate structure
        let instruction = ProgramInstruction::unpack(instruction_data)?;
        match instruction {
            ProgramInstruction::InitializeAccount => Self::initialize_account(accounts),
            ProgramInstruction::MintToAccount { key, value } => {
                Self::mint_keypair_to_account(accounts, key, value)
            }
            ProgramInstruction::TransferBetweenAccounts { key } => {
                Self::transfer_keypair_to_account(accounts, key)
            }
            ProgramInstruction::BurnFromAccount { key } => {
                Self::burn_keypair_from_account(accounts, key)
            }
        }
    }
}
