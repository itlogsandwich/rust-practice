use std::io;
use std::io::Write;

pub fn octal_conversion()
{
    println!("Enter octal");

    let mut oct = String::new();

    io::stdin()
        .read_line(&mut oct)
        .expect("Reading Error");

    io::stdout()
        .flush()
        .expect("Flushing Error");

    let oct = match oct.trim()
    {
        _ if oct.trim().chars().count() <= 3 => oct,
        _ =>
        {
            println!("Max 3 length!");
            return;
        },
    };

    println!("{oct}");

}
