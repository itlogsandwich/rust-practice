use crate::error::Error;
pub struct Todo
{
    pub description: String,
    pub is_done: bool,
}

impl Todo
{
    pub fn new(description: String) -> Self
    {
        Self 
        {
            description,
            is_done: false,
        }
    }

    pub fn mark_as_done(&mut self) -> Result<(), Error>
    {
        if self.is_done
        {
            return Err(Error::AlreadyDone);
        }

        self.is_done = true;
        Ok(())
    
    }
}
