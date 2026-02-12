use crate::decimal_operations;
use std::io;
use std::io::Write;


pub fn octal_val(b: u8, x: usize) -> i32
{
    let val: i32 = b.into();

    val.pow(x as u32)
}

pub fn octal_to_decimal(oct: &str) -> i32
{

    let mut dec = 0;

    for (x, byte) in oct.trim().bytes().enumerate()
    {
        match byte
        {
            b'0'..=b'7' =>
            {
                let b = byte - 0x30;
                dec += octal_val(b, x);
            },
            _ => continue,
        };
    }

    dec
}

pub fn octal_to_binary(oct: &str)
{
    let dec = octal_to_decimal(oct);
    decimal_operations::decimal_to_binary(dec); 
}

pub fn octal_to_hexadecimal(oct: &str)
{
    let dec = octal_to_decimal(oct);
    decimal_operations::decimal_to_hexadecimal(dec);
}

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
        _ if oct.len() == 3 => oct,
        _ if oct.len() < 3 =>
        {
            let mut s = String::new();

            let count = oct.len();

            let difference = 3 - count;

            let mut x = 0;

            while x < difference
            {
                s.push('0');
                x += 1;
            }

            s.push_str(&oct);

            s
        }
        _ =>
        {
            println!("Max 3 length!");
            return;
        },
    };

    for x in oct.trim().bytes()
    {
        match x 
        {
            b'0'..=b'7' => continue,
            _ => return,
        };
    }

    println!("==================");
    print!("Octal: {}", &oct);
    println!("Decimal: {}", octal_to_decimal(&oct));
    octal_to_binary(&oct);
    octal_to_hexadecimal(&oct);
    println!("==================");
}
