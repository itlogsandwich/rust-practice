use crate::decimal_operations;
use std::io;
use std::io::Write;

fn binary_val(dec: i32, x: usize) -> i32
{
    let mut val = dec;
    let arr = [128, 64, 32, 16, 8, 4, 2 ,1];   

    val += arr[x];

    return val;
}

fn binary_to_decimal(bin: &String) -> i32
{
    let characters: Vec<char> = bin.chars().collect();

    let mut dec = 0;

    for (x, char) in characters.iter().enumerate()
    {
        match char
        {
            '1' => dec = binary_val(dec, x),
            _ => continue,
        };
    }

    return dec;
}

fn binary_to_octal(bin: &String)
{
    let dec = binary_to_decimal(bin);
    decimal_operations::decimal_to_octal(dec); 
}

fn binary_to_hexadecimal(bin: &String)
{

    let dec = binary_to_decimal(bin);
    decimal_operations::decimal_to_hexadecimal(dec);
}

pub fn binary_conversion()
{
    println!("Enter binary");

    let mut bin = String::new();

    io::stdin()
        .read_line(&mut bin)
        .expect("Reading Error");

    io::stdout()
        .flush()
        .expect("Flushing Error");

    let bin = match bin.trim()
    {
        _ if bin.trim().chars().count() <= 8 => bin,
        _ =>
        {
            println!("Max 8 length");
            return;
        }
    };

    for x in bin.trim().chars()
    {
        match x 
        {
            '0' => continue,
            '1' => continue,
            _ =>
            {
                println!("INVALID BINARY");
                return; 
            },
        };
    }

    println!("==================");
    print!("Binary: {}", &bin);
    println!("Decimal: {}",binary_to_decimal(&bin));
    binary_to_octal(&bin);
    binary_to_hexadecimal(&bin);
    println!("==================");
}
