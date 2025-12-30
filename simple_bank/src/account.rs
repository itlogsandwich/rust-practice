use crate::error::Error;

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
    
    pub fn check_pin(&self, pin: u64) -> Result<bool, Error>
    {
        if self.pin != pin
        {
            return Ok(true); 
        }
        else
        {
            return Err(Error::InvalidCredentials);
        }
    }
    pub fn update_balance(&mut self, money: u64)
    {
        self.balance += money;
    }
}
