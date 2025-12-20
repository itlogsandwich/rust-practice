use std::io;
use std::fmt;

#[derive(Debug, PartialEq)]
enum Error
{
    InvalidEmail,
    IncorrectPwd,
}

#[derive(Clone)]
struct Email
{
    email: String,
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Error::InvalidEmail => write!(f, "This is not a valid email address!"),
            Error::IncorrectPwd => write!(f, "Passwords do not match!"),
        }
    }
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
    pub fn new(email: String) -> Result<Self, Error>
    {
        let trimmed = email.trim();

        match trimmed.split_once("@")
        {
            Some((prefix, domain)) if domain.contains(".") => 
            { 
                Ok(Self { email })
            }
            _=> 
            {
                Err(Error::InvalidEmail)
            },
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
            email,
            pwd,
        }
    }

    pub fn get_email(&self) -> &Email
    {
        &self.email
    }

    pub fn get_pwd(&self) -> &str
    {
        &self.pwd
    }
}

fn login_display(user: &User)
{

    println!();
    println!("Welcome {}", user.get_email().email);

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

                println!("==========");
                println!("LOGIN!"); 
                println!("==========");

                println!();

                println!("Enter Email: ");
                let mut email = String::new();

                io::stdin()
                    .read_line(&mut email)
                    .expect("Input Error");

                let email = match email.trim().split_once("@")
                {
                    Some((prefix, domain)) if domain.contains(".") => email,
                    _=> 
                    {
                        println!("Error {}", Error::InvalidEmail);
                        println!();
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
                

                if let Some(logged_user) = user_list.iter().find(|user| user.get_email().email == email && user.get_pwd() == pwd.as_str())
                {
                    login_display(logged_user);
                }
                else
                {
                    println!("Invalid Credentials");
                } 
            },

            2 =>
            {
                println!("==========");
                println!("REGISTER!"); 
                println!("==========");

                println!();
                println!("Enter Email: ");
                let mut email = String::new();

                io::stdin()
                    .read_line(&mut email)
                    .expect("Input Error");

                let email = Email::new(email);

                let valid_email = match email
                {
                    Ok(val) => val,
                    Err(e) =>
                    {
                        println!("Error: {e}");
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
              
                println!();
                println!("Confirm Password");

                let mut confirmed_pwd = String::new();

                io::stdin()
                    .read_line(&mut confirmed_pwd)
                    .expect("Input Error");

                confirmed_pwd = confirmed_pwd.trim().to_string();

                if confirmed_pwd == pwd
                {
                    let user = User::new(valid_email, confirmed_pwd);
                    user_list.push(user);
                    println!();
                }
                else
                {
                    println!("Error: {}", Error::IncorrectPwd);
                }
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
