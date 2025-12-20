
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

    pub fn mark_as_done(&mut self)
    {
        self.is_done = true
    }
}
