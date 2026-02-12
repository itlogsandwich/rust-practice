use crate::decimal_operations;
use std::io;
use std::io::Write;

fn binary_val(dec: i32, x: usize) -> i32
{
    let val = dec;

    val.pow(x as u32)
}

fn binary_to_decimal(bin: &str) -> i32
{
    let mut dec = 0;

    for (x, byte) in bin.trim().bytes().enumerate()
    {
        match byte
        {
            b'1' => dec = binary_val(dec, x),
            _ => continue,
        };
    }

    dec
}

fn binary_to_octal(bin: &str)
{
    let dec = binary_to_decimal(bin);
    decimal_operations::decimal_to_octal(dec); 
}

fn binary_to_hexadecimal(bin: &str)
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
        _ if bin.len() == 8 => bin,
        _ if bin.len() < 8  =>
        {
            let mut s = String::new();

            let count = bin.len();

            let difference = 8 - count;

            let mut x = 0;

            while x < difference
            {
                s.push('0');

                x += 1;
            };

            s.push_str(&bin);

            s
        },
        _ =>
        {
            println!("Max 8 length");
            return;
        }
    };

    for x in bin.trim().bytes()
    {
        match x 
        {
            b'0' => continue,
            b'1' => continue,
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
