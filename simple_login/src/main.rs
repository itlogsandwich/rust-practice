use std::io;
use std::fmt;

#[derive(Clone)]
struct Email
{
    email: String,
}

impl fmt::Display for Email
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.email)
    }
}

impl Email
{
    pub fn new(email: String) -> Self
    {
        Self
        {
            email: email,
        }
    }
}

#[derive(Clone)]
struct User
{
    email: Email,
    pwd: String,
}

impl User
{
    pub fn new(email: Email, pwd: String) -> Self
    {
        Self
        {
            email: email,
            pwd: pwd,
        }
    }

    pub fn get_email(&mut self) -> &Email
    {
        return &self.email;
    }

    pub fn get_pwd(&mut self) -> &str
    {
        return &self.pwd;
    }
}

fn login_display(mut user: User)
{
    println!("Welcome {}", user.get_email());

    println!("SKIBIDI RIZZ");
}

fn main() 
{

    let mut user_list: Vec<User> = Vec::new();

    loop 
    {
        println!("BLAH BLAH PORTAL");

        println!("[1]Login, [2]Register, [3]Exit");
        let mut choice = String::new();
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Input error");

        let choice = match choice.trim().parse::<u8>()
        {
            Ok(choice) => choice,
            Err(_) =>
            {
                println!("Parsing Error");
                continue;
            }
        };

        match choice
        {
            1 =>
            {
                println!("LOGIN!");
 
                println!("Enter Email: ");
                let mut email = String::new();

                io::stdin()
                    .read_line(&mut email)
                    .expect("Input Error");

                let email = match email.trim().to_string()
                {
                    _ if email.contains("@") => email,
                    _ =>
                    {
                        println!("Invalid email");
                        continue;
                    },
                };

                println!();

                println!("Enter password");
                let mut pwd = String::new();

                io::stdin()
                    .read_line(&mut pwd)
                    .expect("Input Error");

                pwd = pwd.trim().to_string();

                if pwd.len() < 8 
                { 
                    println!("Password must contain at least 8 characters");
                    continue;
                }
                
                for user in &mut user_list
                {
                    match user
                    {
                        _ if user.get_email().email == email && user.get_pwd() == pwd =>
                        {
                            let logged_user = user.clone();

                            println!();
                            println!("Successful Login!");
                            login_display(logged_user);
                            break;
                        },
                        _ =>
                        {
                            println!("User does not exist!");
                        },
                    };
                }
            },

            2 =>
            {
                println!("REGISTER!");
               
                println!("Enter Email: ");
                let mut email = String::new();

                io::stdin()
                    .read_line(&mut email)
                    .expect("Input Error");

                let email = match email.trim().to_string()
                {
                    _ if email.contains("@") => email,
                    _ =>
                    {
                        println!("Invalid email");
                        continue;
                    },
                };

                println!();

                println!("Enter password");
                let mut pwd = String::new();

                io::stdin()
                    .read_line(&mut pwd)
                    .expect("Input Error");

                pwd = pwd.trim().to_string();

                if pwd.len() < 8 
                { 
                    println!("Password must contain at least 8 characters");
                    continue;
                }

                let user = User::new(Email::new(email), pwd);

                user_list.push(user);
            },

            3 => 
            {
                println!("Exiting...");
                break;
            },
            _ => println!("INVALID INPUT"),
        };
    }
}
