use crate::account::Account;
use crate::error::Error;
use std::collections::HashMap;

pub struct Bank
{
    accounts: HashMap<String, Account>,
}

impl Bank
{
    pub fn create_account(&mut self, owner: String, pin: u64) -> Result<String, Error>
    {
        if owner.is_empty()
        {
            return Err(Error::InvalidName);
        }

        let acc_num = format!("{:04}", self.accounts.len() + 1000);

        let account = Account::new(acc_num.clone(), owner, pin);

        self.accounts.insert(acc_num.clone(), account);

        Ok(acc_num)

    }

    pub fn deposit(&mut self, acc_num: String, money: u64) -> Result<(), Error>
    {
        if money == 0
        {
            return Err(Error::InvalidDeposit);
        }

        if let Some(acc) = self.accounts.get_mut(&acc_num)
        {
            acc.update_balance(money);
        }
        else
        {
            return Err(Error::NotFound);
        }

        Ok(())

    }
}


