use crate::todo::Todo;
use crate::error::Error;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList
{
    todos: Vec<Todo>,
}

impl TodoList
{
    pub fn new() -> Self
    {
        Self { todos: Vec::new() }
    }
}

impl TodoList
{
    pub fn save(&self, path: &str) -> Result<(), Error>
    {
        let serialized = serde_json::to_string(&self)?;
        
        fs::write(path, serialized)?;

        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, Error>
    {
        let serialized = fs::read_to_string(path)?;

        let deserialized: TodoList = serde_json::from_str(&serialized)?;
        
        Ok(deserialized)
    }

    pub fn add(&mut self, description:String)
    {
        let todo = Todo::new(description);
        self.todos.push(todo);
    }

    pub fn update(&mut self, index: usize) -> Result<(), Error>
    {
        if index == 0
        {   
            return Err(Error::InvalidIndex);
        }
        match self.todos.get_mut(index - 1)
        {
            Some(val) =>
            {
                val.mark_as_done()?;

                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }

    pub fn list(&self)
    {
        for (nth, todo) in self.todos.iter().enumerate()
        {
            println!("{} Todo: {}  Done: {}", nth + 1,  todo.description, todo.is_done);
        }
    }

    pub fn list_finished(&self)
    {
        self.todos.iter()
            .filter(|todo| todo.is_done)
            .enumerate()
            .for_each(|(nth, todo)|
            {
                println!("{} Todo: {}  Done: {}", nth + 1,  todo.description, todo.is_done);
            })
    }

    pub fn length(&self) -> usize { self.todos.len() }

}
