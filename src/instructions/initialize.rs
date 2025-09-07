use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_system::instructions::CreateAccount;

use crate::{Counter, ID};

pub struct InitializeAccounts<'a> {
    pub counter: &'a AccountInfo,
    pub user: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitializeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [counter, user, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            counter,
            user,
            system_program,
        })
    }
}

pub struct Initialize<'a> {
    pub accounts: InitializeAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Initialize<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = InitializeAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'a> Initialize<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        // let lamports = Rent::minimum_balance(size);
        let lamports = 10_000_000; //find way to get rent

        CreateAccount {
            from: &self.accounts.user,
            to: &self.accounts.counter,
            lamports,
            space: Counter::LEN as u64,
            owner: &ID,
        }
        .invoke()?;

        let mut byte_data = self.accounts.counter.try_borrow_mut_data()?;
        let counter = Counter::load_mut(byte_data.as_mut())?;
        counter.count = 0;

        Ok(())
    }
}
