use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Account
{
    account_number: String,
    owner: String,
    pin: String,
    balance: u64,
}

impl Account
{
    pub fn new(account_number: String, owner: String, pin: String) -> Self
    {
        Self
        {
            account_number,
            owner,
            pin,
            balance: 0,
        }
    } 

    pub fn check_pin(&self, pin: &str) -> Result<(), Error>
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

    pub fn get_owner(&self) -> &str
    {
       &self.owner 
    }
    pub fn add_balance(&mut self, money: u64)
    {
        self.balance += money;
    }
    
    pub fn deduct_balance(&mut self, money: u64)
    {
        self.balance -= money;
    }

    pub fn get_account_number(&self) -> &str
    {
        &self.account_number
    }
    pub fn get_balance(&self) -> u64
    {
        self.balance
    }
}
