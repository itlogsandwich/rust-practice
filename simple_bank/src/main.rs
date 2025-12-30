use std::io;
use crate::bank::Bank;
use crate::account::Account;
mod account;
mod bank;
mod error;

fn menu(acc: &Account)
{
    println!("Welcome, {}", acc.get_owner());   
}

fn main() 
{
     
    let mut bank = Bank::new();

    println!("Bank");

    loop
    {
        println!("[1]Login\n[2]Register\n[3]Exit");
        let mut choice = String::new();


        io::stdin()
            .read_line(&mut choice)
            .expect("Input error");
        
        let choice = match choice.trim().parse::<u64>()
        {
            Ok(val) => val,
            Err(e) => 
            {
                println!("Parsing failed {e}");
                break;
            }
        };

        match choice
        {
            1 =>
            {
                println!("Enter account number");
                let mut acc_num = String::new();

                io::stdin()
                    .read_line(&mut acc_num)
                    .expect("Input error");


                println!("Enter pin");
                let mut pin = String::new();

                io::stdin()
                    .read_line(&mut pin)
                    .expect("Input error");
                
                let pin = match pin.trim().parse::<u64>()
                {
                    Ok(val) => val,
                    Err(e) => 
                    {
                        println!("Parsing failed {e}");
                        break;
                    }
                };

                match bank.auth(&acc_num, pin)
                {
                    Ok(val) => menu(val),
                    Err(e) => println!("Error: {e}"),
                }
            }

            2 =>
            {
                todo!("REGISTER STUFF");
            }

            3 => break,

            _ => println!("INVALID OPTION!"),
        };
    }
}
