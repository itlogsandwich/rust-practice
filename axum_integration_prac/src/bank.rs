use crate::account::Account;
use crate::error::Error;
use std::collections::HashMap;

pub type BankResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
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
    pub fn auth(&self, acc_num: &str, pin: &str) -> BankResult<&Account>
    { 
        let acc = self.accounts.get(acc_num)
                    .ok_or(Error::NotFound)?;

        acc.check_pin(pin)?;

        Ok(acc)
    }

    pub fn create_account(&mut self, owner: String, pin: String) -> BankResult<String>
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

    pub fn display_owner(&self, acc_num: &str) -> BankResult<&str>
    {
        let owner = self.accounts.get(acc_num)
                .map(|acc| acc.get_owner())
                .ok_or(Error::NotFound)?;

        Ok(owner)
    }

    pub fn check_balance(&self, acc_num: &str) -> BankResult<u64>
    {
        let balance = self.accounts.get(acc_num)
                .map(|acc| acc.get_balance())
                .ok_or(Error::NotFound)?;

        Ok(balance)
    }

    pub fn deposit(&mut self, acc_num: &str, money: u64) -> BankResult<()>
    {
        if money == 0
        {
            return Err(Error::InvalidDeposit);
        }

        self.accounts.get_mut(acc_num)
            .ok_or(Error::NotFound)?
            .add_balance(money)?;

        Ok(())
    }

    pub fn withdraw(&mut self, acc_num: &str, money: u64) -> BankResult<()>
    {
        if money == 0
        {
            return Err(Error::InvalidWithdrawal);
        }

        self.accounts.get_mut(acc_num)
            .ok_or(Error::NotFound)?
            .deduct_balance(money)?;

        Ok(())
    }
}


