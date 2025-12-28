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

    pub fn update_balance(&mut self, money: u64)
    {
        self.balance += money;
    }
}
