// Generate the nth Fibonacci number
use std::io;

fn main() {
    let mut n = String::new();

    println!("Please input n for printing nth Fibonacci number");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse()
        .expect("Please enter valid input");


    let mut a = 0;
    let mut b = 1;
    if n <= 0 {
        println!("Invalid input!");
    } else if n == 1 {
        println!("1st Fibonacci number: {b}"); 
    } else {
        for _ in 2..n {
            let c = a + b;
            a = b;
            b = c;
        }
        println!("{n}th Fibonacci number: {b}"); 
    }
}