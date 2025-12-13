use crate::decimal_operations;
use std::io;
use std::io::Write;


pub fn octal_val(b: u8, x: usize) -> i32
{
    let mut val: i32 = b.into();
    let arr = [64, 8, 1];

    val *= arr[x];
    return val;
}

pub fn octal_to_decimal(oct: &String) -> i32
{

    let mut dec = 0;

    for (x, byte) in oct.bytes().enumerate()
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

    return dec;
}

pub fn octal_to_binary(oct: &String)
{
    let dec = octal_to_decimal(oct);
    decimal_operations::decimal_to_binary(dec); 
}

pub fn octal_to_hexadecimal(oct: &String)
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
        _ if oct.trim().bytes().count() == 3 => oct,
        _ if oct.trim().bytes().count() < 3 =>
        {
            let mut s = String::new();

            let count = oct.trim().bytes().count();

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
