use std::io::Write;
use std::io;


fn fahrenheit_to_celsius()
{
    println!("Enter a temperature in Fahrenheit!");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Invalid!");

    io::stdout()
        .flush()
        .expect("Invalid");

    println!("Current Temperature\nFahrenheit: {}", &mut temperature);

    let temperature:f64 = temperature.trim().parse::<f64>().expect("Invalid parsing!");
   
    let celsius: f64 = (temperature - 32.0) * 5.0 / 9.0;

    println!("Celsius: {}", celsius);

}

fn celsius_to_fahrenheit()
{
    println!("Enter a temperature in Celsius");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Invalid!");
    
    io::stdout()
        .flush()
        .expect("Invalid");
    
    println!("Current Temperature\nCelsius: {}", &mut temperature);

    let temperature = temperature.trim().parse::<f64>().expect("Invalid parsing!");

    let fahrenheit: f64 = (temperature * 9.0) / 5.0 + 32.0;

    println!("Fahrenheit: {}", fahrenheit);

}
fn main() 
{

    loop 
    {
        println!("Temperature Converter\n[1]Fahrenheit to Celsius\n[2]Celsius to Fahrenheit\n[3]Exit");
        let mut choice = String::new();
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Invalid!");
    
        io::stdout()
            .flush()
            .expect("Invalid");

        let choice: u32 = match choice.trim().parse::<u32>()
        {
            Ok(num) => num,
            Err(_) => 
            {
                println!("Invalid input! Try again");
                continue;
            }
        };
        match choice 
        {
            1 => fahrenheit_to_celsius(),
            2 => celsius_to_fahrenheit(),
            3 => break,
            _ => println!("Invalid choice!"),
        };
    };
}

