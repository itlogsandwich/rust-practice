use crate::account::Account;
use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bank
{
    accounts: HashMap<String, Account>,
}

impl Bank
{
    pub fn new() -> Self
    {
        Self { accounts: HashMap::new() }
    }
}

impl Bank
{
    pub fn auth(&self, acc_num: &str, pin: &str) -> Result<&Account, Error>
    { 
        let acc = self.accounts.get(acc_num)
                    .ok_or(Error::NotFound)?;

        acc.check_pin(pin)?;

        Ok(acc)
    }

    pub fn create_account(&mut self, owner: String, pin: String) -> Result<String, Error>
    {
        if owner.is_empty()
        {
            return Err(Error::InvalidCredentials);
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

    pub fn proof(&self, acc_num: &str) -> bool
    {
        self.accounts.contains_key(acc_num)
    }
}


