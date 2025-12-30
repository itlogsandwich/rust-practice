use crate::error::Error;

#[derive(Debug)]
pub struct Account
{
    account_number: String,
    owner: String,
    pin: u64,
    balance: u64,
}

impl Account
{
    pub fn new(account_number: String, owner: String, pin: u64) -> Self
    {
        Self
        {
            account_number,
            owner,
            pin,
            balance: 0,
        }
    } 

    pub fn check_pin(&self, pin: u64) -> Result<(), Error>
    {
        if self.pin == pin
        {
            Ok(())
        }
        else
        {
            Err(Error::InvalidCredentials)
        }
    }

    pub fn update_balance(&mut self, money: u64)
    {
        self.balance += money;
    }
    
    pub fn get_account_number(&self) -> &str
    {
        &self.account_number
    }
    pub fn get_balance(&self) -> u64
    {
        self.balance
    }

    pub fn get_owner(&self) -> &str
    {
        &self.owner
    }
}
