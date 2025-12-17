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
    pub fn add(&mut self, description:String, )
    {
        let todo = Todo::new(description);
        self.todos.push(todo);
    }

    pub fn list(&mut self)
    {
        for (_x, todo) in self.todos.iter().enumerate()
        {
            println!("Todo: {}  Done: {}", todo.description, todo.is_done);
        }
    }
}
