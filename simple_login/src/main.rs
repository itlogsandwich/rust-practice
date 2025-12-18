use std::io;
use std::fmt;

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
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

    pub fn get_email(&self) -> &Email
    {
        return &self.email;
    }

    pub fn get_pwd(&self) -> &str
    {
        return &self.pwd;
    }
}

fn login_display(user: Option<&User>)
{
    if let Some(logged_user) = user
    {            
        println!("Welcome {}", logged_user.get_email().email);
    }
    else
    {
        return;
    }

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
                

                let logged_user = user_list.iter().find(|user| user.get_email().email == email && user.get_pwd() == pwd.as_str());
 
                login_display(logged_user);
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
