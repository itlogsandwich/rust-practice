use crate::todo::Todo;

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
    pub fn add(&mut self, description:String)
    {
        let todo = Todo::new(description);
        self.todos.push(todo);
    }

    pub fn update(&mut self, index: usize) -> Result<(), String>
    {
        if index == 0
        {   
            return Err(String::from("Invalid index!"));
        }
        match self.todos.get_mut(index - 1)
        {
            Some(val) =>
            {
                val.mark_as_done()?;

                Ok(())
            }
            None => Err(String::from("Task not found")),
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
