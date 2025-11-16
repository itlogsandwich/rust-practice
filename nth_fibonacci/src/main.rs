use std::io;
use std::io::Write;


fn generate_fibo(n: i32)
{
    if n <= 1
    {
        println!("Fibonacci: {}", n);
    }
    else 
    {   
        let mut lo = 0;
        let mut hi = 1;
        let mut res = 0;
        for _x in 1..n
        {
           res = lo + hi;
           lo = hi;
           hi = res;
        }

        println!("Fibonacci {}", res);
    }
    
}
fn main() 
{
    println!("Enter n to find the nth Fibonacci Value");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Reading Error");

    io::stdout()
        .flush()
        .expect("Flushing Error");

    let n = match n.trim().parse::<i32>()
    {
        Ok(n) => n,
        Err(_) => 0,
    };

    generate_fibo(n);
}
