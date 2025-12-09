use std::io;
use std::io::Write;

fn decimal_to_binary(mut x: i32)
{
    let mut vec: Vec<i32> = Vec::new();

    while x > 0
    {
        if x % 2 == 0
        {
            vec.push(0);
        }

        vec.push(1);

        x = x / 2;
    }

    print!("Binary: ");
    while let Some(top) = vec.pop()
    {
        print!("{top}");
    }

    println!();
}

fn decimal_to_octal(mut x: i32)
{
    let mut vec: Vec<i32> = Vec::new();

    while x > 0
    {
        if x % 8 == 0
        {
            vec.push(0); 
        }
        let remainder = x % 8;
        vec.push(remainder);

        x = x / 8;
    }

    print!("Octal: ");
    while let Some(top) = vec.pop()
    {
        print!("{top}");
    }

    println!();
}

fn decimal_to_hexadecimal(x: i32)
{
    let mut x: u8 = x.try_into().unwrap();
    let mut vec: Vec<char> = Vec::new();

    while x > 0
    {
        if x % 16 == 0
        {
            vec.push(0 as char);
        }

        let remainder = x % 16;
        
        match remainder 
        {
            10 => vec.push('A'),
            11 => vec.push('B'),
            12 => vec.push('C'),
            13 => vec.push('D'),
            14 => vec.push('E'),
            15 => vec.push('F'),
            _ => vec.push((remainder + 0x30) as char),
        };

        x = x / 16;
    }

    print!("Hexadecimal: ");
    while let Some(top) = vec.pop()
    {
        print!("{top}");
    }

    println!();
}

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
    decimal_to_octal(dec); 
}

fn binary_to_hexadecimal(bin: &String)
{

    let dec = binary_to_decimal(bin);
    decimal_to_hexadecimal(dec);
}

fn octal_conversion()
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

}
fn binary_conversion()
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

fn decimal_conversions()
{
    println!("Enter number: ");
    
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Reading Error");

    io::stdout()
        .flush()
        .expect("Flushing Error");

    let num = match num.trim().parse::<i32>()
    {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("==================");
    println!("Decimal: {num}");
    decimal_to_binary(num);
    decimal_to_octal(num);
    decimal_to_hexadecimal(num); 
    println!("==================");
}

fn main() 
{
    loop 
    {
        println!("Number Converter!\n[1]Decimal to ...\n[2]Binary to ...\n[3]Octal to ...\n[4]Hexadecimal to ...\n[5]Exit");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Reading Error");

        io ::stdout()
            .flush()
            .expect("Flushing Error");

        let choice = match choice.trim().parse::<u8>()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("Parsing Error");
                continue;
            },
        };

        match choice 
        {
            1 =>
            {
                decimal_conversions();
                break;
            },

            2 =>
            {
                binary_conversion();
                break;
            }

            3 =>
            {
                break;
            }

            4 =>
            {
                break;
            }

            5 => break,
            _ => println!("Invalid Choice"),
        };
    }
}
