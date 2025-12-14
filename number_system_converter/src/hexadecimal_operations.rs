use crate::decimal_operations;
use std::io;
use std::io::Write;

pub fn hexadecimal_val(b: u8, x: usize) -> i32
{
    let mut val = match b
    {
        b'A' => 10,
        b'B' => 11,
        b'C' => 12,
        b'D' => 13,
        b'E' => 14,
        b'F' => 15,
        _ => b,
    };

    let arr = [16, 1];

    val *= arr[x];
    return val.into();
}

pub fn hexadecimal_to_decimal(hex: &String) -> i32
{
    let mut dec = 0;

    for (x, byte) in hex.bytes().enumerate()
    {
        match byte
        {
            b'A'..=b'F' => 
            {
                dec += hexadecimal_val(byte, x);
            },
            b'0'..=b'9' =>
            {
                let byte = byte - 0x30;
                
                dec += hexadecimal_val(byte, x);
            },
            _ =>continue,
        }
    }

    return dec;
}

pub fn hexadecimal_to_binary(hex: &String)
{
    let dec = hexadecimal_to_decimal(hex);
    decimal_operations::decimal_to_binary(dec);    
}

pub fn hexadecimal_to_octal(hex: &String)
{
    let dec = hexadecimal_to_decimal(hex);
    decimal_operations::decimal_to_octal(dec);    
}

pub fn hexadecimal_conversion()
{
    println!("Enter hexadecimal");

    let mut hex = String::new();

    io::stdin()
        .read_line(&mut hex)
        .expect("Reading Error");

    io::stdout()
        .flush()
        .expect("Flushing Error");

    let hex = match hex.trim()
    {
        _ if hex.trim().bytes().count() == 2 => hex,
        _ if hex.trim().bytes().count() < 2 =>
        {
            let mut s = String::new();
            
            s.push('\n');

            s.push_str(&hex);

            s
        }
        _ =>
        {
            println!("Max 2 length!");
            return;
        },
    };

    println!("==================");
    print!("Hexadecimal: {}", &hex);
    println!("Decimal: {}", hexadecimal_to_decimal(&hex));
    hexadecimal_to_binary(&hex);
    hexadecimal_to_octal(&hex);
    println!("==================");
}
