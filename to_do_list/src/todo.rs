
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
}
