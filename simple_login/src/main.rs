use std::io;
use std::fmt;

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
}

fn main() 
{
    loop 
    {
        println!("BLAH BLAH PORTAL");

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
        
        login_display(user);    
    }
}
