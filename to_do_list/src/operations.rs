use crate::todo_list::TodoList;
use crate::error::Error;
use std::io;

fn add_todo(todo_list: &mut TodoList)
{
    println!("Enter Task: ");
    
    let mut description = String::new();

    io::stdin()
        .read_line(&mut description)
        .expect("Input Error");

    description = description.trim().to_string();
   
    
    todo_list.add(description);
    println!();
}

fn show_todo(todo_list: &mut TodoList)
{
    println!("Things to do!");
    println!("==========");
    todo_list.list();
    println!("==========");
}

fn show_finished(todo_list: &mut TodoList)
{
    println!("Tasks Accomplished!");
    println!("==========");
    todo_list.list_finished();
    println!("==========");

    println!();
}

fn mark_out(todo_list: &mut TodoList) -> Result<(), Error>
{
    println!("Tasks to do!");
    println!("==========");
    todo_list.list();
    println!("==========");

    println!("Pick to mark as done");
    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("Input Error");

    let task = task.trim().parse::<usize>()?;
    
    todo_list.update(task)
}

pub fn menu()
{
    let mut todo_list = TodoList::new();

    loop
    {
        println!("Todo Tracker");

        show_todo(&mut todo_list);

        println!();

        println!("[1]Add Task\n[2]Show accomplished\n[3]Mark as done\n[4]Exit");
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Input Error");
        
        let input = match input.trim().parse::<u8>()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("Error has occured: {}", Error::ParsingError);
                continue;
            }
        };

        match input
        {
            1 =>
            {
                add_todo(&mut todo_list);
            },

            2 =>
            {
                show_finished(&mut todo_list);
            },

            3 if todo_list.length() > 0  => 
            {
                if let Err(e) = mark_out(&mut todo_list)
                {
                    println!("Error has occured: {}", e);
                }
            },

            4 => break,

            _ => println!("Invalid choice! There must be tasks to do!"),
        };
    }
}
