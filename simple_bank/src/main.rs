use std::io;
use crate::bank::Bank;
use crate::error::Error;
mod account;
mod bank;
mod error;

fn check_pin_length(pin: &str) -> Result<(), Error>
{
    if pin.len() < 8
    {
        Err(Error::PasswordLength)
    }
    else
    {
        Ok(())
    }
}

fn menu(bank: &mut Bank, acc_num: &str) -> Result<(), Error>
{

    match bank.display_owner(&acc_num)
    {
        Ok(name) => println!("Welcome, {name}"),
        Err(e) => println!("{e}"),
    };

    loop
    {
        println!("[1]Check Balance\n[2]Deposit\n[3]Withdraw\n[4]Exit");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Input error");
        
        let choice = match choice.trim().parse::<u8>()
        {
            Ok(val) => val,
            Err(e) =>
            {
                println!("{e}");
                4
            },
        };
        
        match choice
        {
            1 =>
            {
                match bank.check_balance(&acc_num)
                {
                    Ok(bal) => println!("Current Balance: {bal}"),
                    Err(e) => println!("{e}"),
                };
            }
            2 => 
            {
                println!("Insert amount to be deposited");
                let mut money = String::new();

                io::stdin()
                    .read_line(&mut money)
                    .expect("Input error");

                let money = match money.trim().parse::<u64>()
                {
                    Ok(val) => val,
                    Err(e) => 
                    {
                        println!("{e}");
                        0
                    },
                };

                bank.deposit(&acc_num, money)?;
            },
            3 =>
            {
                println!("Insert amount to be withdrawn");
                let mut money = String::new();

                io::stdin()
                    .read_line(&mut money)
                    .expect("Input error");

                let money = match money.trim().parse::<u64>()
                {
                    Ok(val) => val,
                    Err(e) => 
                    {
                        println!("{e}");
                        0
                    },
                };

                bank.withdraw(&acc_num, money)?;

            },
            
            4 => break Ok(()),
            _ => println!("Invalid input!"),
        };
    }

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
        
        let choice = match choice.trim().parse::<u8>()
        {
            Ok(val) => val,
            Err(e) => 
            {
                println!("Parsing failed {e}");
                3
            },
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
                
                if let Err(e) = check_pin_length(pin)
                {
                    println!("{e}");
                    continue;
                }

                match bank.auth(&acc_num, pin)
                {
                    Ok(acc) => 
                    {
                        let user_id = acc.get_account_number().to_string();

                        if let Err(e) = menu(&mut bank, &user_id)
                        {
                            println!("{e}");
                        };
                    },
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

                if let Err(e) = check_pin_length(pin)
                {
                    println!("{e}");
                    continue;
                }

                println!("Confirm pin");
                let mut confirm_pin = String::new();

                io::stdin()
                    .read_line(&mut confirm_pin)
                    .expect("Input error");
                
                let confirm_pin = confirm_pin.trim();

                if let Err(e) = check_pin_length(confirm_pin)
                {
                    println!("{e}");
                    continue;
                }

                if pin == confirm_pin
                {
                    let acc_num = bank.create_account(owner, pin.to_string()).expect("An error has occured");

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
