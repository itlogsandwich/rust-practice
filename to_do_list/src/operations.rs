use crate::todo_list::TodoList;
use crate::error::Error;
use std::io;

fn add_todo(todo_list: &mut TodoList, path: &str) -> Result<(), Error>
{
    println!("Enter Task: ");
    
    let mut description = String::new();

    io::stdin().read_line(&mut description)?;

    description = description.trim().to_string();
   
    
    todo_list.add(description);

    todo_list.save(path)?;

    Ok(())
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

fn mark_out(todo_list: &mut TodoList, path: &str) -> Result<(), Error>
{
    println!("Tasks to do!");
    println!("==========");
    todo_list.list();
    println!("==========");

    println!("Pick to mark as done");
    let mut task = String::new();

    io::stdin().read_line(&mut task)?;

    let task = task.trim().parse::<usize>()?;
    
    todo_list.update(task)?;

    todo_list.save(path)?;

    Ok(())
}

pub fn menu()
{
    let path: &str = "tasks.json";
    
    let mut todo_list = if std::path::Path::new(path).exists()
    {
        match TodoList::load(path)
        {
            Ok(val) => val,
            Err(e) =>
            {
                println!("Error has occured: {e}");
                TodoList::new()
            },
        }
    }
    else
    {
        TodoList::new()
    };

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
            Err(e) =>
            {
                println!("Error has occured: {e}");
                continue;
            }
        };

        match input
        {
            1 =>
            {
                if let Err(e) = add_todo(&mut todo_list, path)
                {
                    println!("Error has occured: {e}");
                }
            },

            2 =>
            {
                show_finished(&mut todo_list);
            },

            3 if todo_list.length() > 0  => 
            {
                if let Err(e) = mark_out(&mut todo_list, path)
                {
                    println!("Error has occured: {e}");
                }
            },

            4 => break,

            _ => println!("Invalid choice! There must be tasks to do!"),
        };
    }
}
