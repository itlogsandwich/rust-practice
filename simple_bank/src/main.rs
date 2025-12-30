use std::io;
use crate::bank::Bank;
use crate::account::Account;
use crate::error::Error;
mod account;
mod bank;
mod error;

fn check_pin_length(pin: &str) -> Result<bool, Error>
{
    if pin.len() < 8
    {
        Err(Error::PasswordLength)
    }
    else
    {
        Ok(true)
    }

}

fn menu(acc: &Account)
{
    println!("Welcome, {}", acc.get_owner());   
}

fn main() 
{
     
    let mut bank = Bank::new();

    println!("Bank of the Sus Islands");

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

                let acc_num = acc_num.trim();

                println!("Enter pin");
                let mut pin = String::new();

                io::stdin()
                    .read_line(&mut pin)
                    .expect("Input error");
                
                let pin = pin.trim();

                match bank.auth(acc_num, pin)
                {
                    Ok(val) => menu(val),
                    Err(e) => println!("Error: {e}"),
                }
            }

            2 =>
            {
                println!("Register Account");
                let mut owner = String::new();

                io::stdin()
                    .read_line(&mut owner)
                    .expect("Input error");


                println!("Enter pin");
                let mut pin = String::new();

                io::stdin()
                    .read_line(&mut pin)
                    .expect("Input error");
                
                let pin = pin.trim();

                match check_pin_length(pin)
                {
                    Ok(true) => println!("Valid"),
                    Ok(false) => continue,
                    Err(e) => 
                    {
                        println!("Error: {e}");
                        continue;
                    },
                };

                println!("Confirm pin");
                let mut confirm_pin = String::new();

                io::stdin()
                    .read_line(&mut confirm_pin)
                    .expect("Input error");
                
                let confirm_pin = confirm_pin.trim();

                if pin == confirm_pin
                {
                    let acc_num = bank.create_account(owner, pin.to_string()).expect("An error has occured");

                    println!("{}", bank.proof(&acc_num));
                
                    println!("Welcome! You're account number is: {}", acc_num);
                }
                else
                {
                    println!("{}", Error::NotMatching);
                }
            }

            3 => break,

            _ => println!("INVALID OPTION!"),
        };
    }
}
