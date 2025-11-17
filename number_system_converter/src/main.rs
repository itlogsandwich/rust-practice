use std::io;
use std::io::Write;



fn decimal_to_binary(mut x: i32)
{
    let mut vec: Vec<i32> = Vec::new();

    while x > 0
    {
        if x % 2 == 0
        {
            vec.push(0);
        }
        else 
        {
            vec.push(1);
        }

        x = x / 2;
    }

    for i in vec.into_iter().rev()
    {
        print!("{i}");
    }
}

fn main() 
{
    println!("Enter number: ");
    
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Reading Error");
    
    io ::stdout()
        .flush()
        .expect("Flushing Error");


    let num = match num.trim().parse::<i32>()
    {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    decimal_to_binary(num);
    
}
