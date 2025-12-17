use crate::todo_list::TodoList;
use std::io;

fn add_todo()
{
    let mut todo_list = TodoList::new();
    print!("Enter Task: ");
    
    let mut description = String::new();

    io::stdin()
        .read_line(&mut description)
        .expect("Input error");

    description = description.trim().to_string();
   
    
    todo_list.add(description); 
}

pub fn menu()
{
    loop
    {
        println!("Todo Tracker");
        
        println!();
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Input Error");
        
        let input = match input.trim().parse::<u8>()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("Parsing error");
                return;
            }
        };

        match input
        {
            1 =>
            {
                add_todo();
                break;
            },

            2 =>
            {
                break;
            },

            3 => 
            {
                break;
            },

            4 =>
            {
                break;
            },

            5 => break,

            _ => println!("Invalid choice!"),
        };
    }
}
