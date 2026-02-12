use std::io;
use std::io::Write;

pub fn decimal_to_binary(mut x: i32)
{
    let mut s = String::new();

    while x > 0
    {
        if x % 2 == 0
        {
            s.push('0');
        }
        else
        {
            s.push('1');
        }

        x /= 2;
    }

    let count = s.len();
    let difference = 8 - count;

    let mut x = 0;

    while x < difference
    {
        s.push('0');

        x += 1;
    }
    
    print!("Binary: ");
    while let Some(top) = s.pop()
    {
        print!("{top}");
    }

    println!();
}

pub fn decimal_to_octal(mut x: i32)
{
    let mut s = String::new();

    if x == 0
    {
        s.push('0');
    }

    while x > 0
    {
        if x % 8 == 0
        {
            s.push('0'); 
        }
        else 
        { 
            let remainder = x % 8;
            s.push((remainder as u8+ 0x30) as char);
        }

        x /= 8;
    }

    print!("Octal: ");
    while let Some(top) = s.pop()
    {
        print!("{top}");
    }

    println!();
}

pub fn decimal_to_hexadecimal(x: i32)
{
    let mut x: u8 = x.try_into().unwrap();
    let mut s = String::new();

    if x == 0
    {
        s.push('0');
    }

    while x > 0
    {
        if x.is_multiple_of(16)
        {
            s.push('0');
        }
        else
        {
            let remainder = x % 16;
            
            match remainder 
            {
                10 => s.push('A'),
                11 => s.push('B'),
                12 => s.push('C'),
                13 => s.push('D'),
                14 => s.push('E'),
                15 => s.push('F'),
                _ => s.push((remainder + 0x30) as char),
            };
        }

        x /= 16;
    }

    print!("Hexadecimal: ");
    while let Some(top) = s.pop()
    {
        print!("{top}");
    }

    println!();
}

pub fn decimal_conversions()
{
    println!("Enter decimal: ");
    
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Reading Error");

    io::stdout()
        .flush()
        .expect("Flushing Error");

    let num:i32 = num.trim().parse::<i32>().unwrap_or_default();

    println!("==================");
    println!("Decimal: {num}");
    decimal_to_binary(num);
    decimal_to_octal(num);
    decimal_to_hexadecimal(num); 
    println!("==================");
}
