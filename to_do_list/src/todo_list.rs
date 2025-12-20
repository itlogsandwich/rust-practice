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

    pub fn update(&mut self, index: usize)
    {
        let todo = &mut self.todos[index - 1];

        todo.mark_as_done();
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
        for (nth, todo) in self.todos.iter().enumerate()
        {
            println!("{} Todo: {}  Done: {}", nth + 1,  todo.description, todo.is_done == true);
        }
    }

    pub fn length(&self) -> usize
    {
        self.todos.len()
    }
}
