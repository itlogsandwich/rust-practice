use crate::account::Account;
use crate::error::Error;
pub struct AccountList
{
    accounts: Vec<Account>,
}

impl AccountList
{
    pub fn add_account(&mut self, account: Account) -> Result<(), Error>
    {
        self.accounts.push(account);

        Ok(())
    }
}


