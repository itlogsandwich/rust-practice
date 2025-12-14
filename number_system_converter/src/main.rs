mod decimal_operations;
mod binary_operations;
mod octal_operations;
mod hexadecimal_operations;

use std::io;
use std::io::Write;

fn main() 
{
    loop 
    {
        println!("Number Converter!\n[1]Decimal to ...\n[2]Binary to ...\n[3]Octal to ...\n[4]Hexadecimal to ...\n[5]Exit");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Reading Error");

        io ::stdout()
            .flush()
            .expect("Flushing Error");

        let choice = match choice.trim().parse::<u8>()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("Parsing Error");
                continue;
            },
        };

        match choice 
        {
            1 =>
            {
                decimal_operations::decimal_conversions();
                break;
            },

            2 =>
            {
                binary_operations::binary_conversion();
                break;
            }

            3 =>
            {
                octal_operations::octal_conversion();
                break;
            }

            4 =>
            {
                hexadecimal_operations::hexadecimal_conversion();
                break;
            }

            5 => break,
            _ => println!("Invalid Choice"),
        };
    }
}
