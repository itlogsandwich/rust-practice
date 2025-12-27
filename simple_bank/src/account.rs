pub struct Account
{
    username: String,
    pin: u64,
    balance: u64,
}

impl Account
{
    pub fn new(username: String, pin: u64) -> Self
    {
        Self
        {
            username,
            pin,
            balance: 0,
        }
    } 
}
