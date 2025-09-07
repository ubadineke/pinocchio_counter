use crate::Counter;
use pinocchio::{
    account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult
};

pub struct IncrementAccounts<'a> {
    pub counter: &'a AccountInfo,
    pub user: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for IncrementAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [counter, user] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self { counter, user })
    }
}

pub struct Increment<'a> {
    pub accounts: IncrementAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Increment<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = IncrementAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'a> Increment<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&mut self) -> ProgramResult {
        msg!("Instruction: Increment");
        let mut byte_data = self.accounts.counter.try_borrow_mut_data()?;
        let counter = Counter::load_mut(byte_data.as_mut())?;
        counter.count += 1; //not using checked_add since it's nothing serious
        Ok(())
    }
}
