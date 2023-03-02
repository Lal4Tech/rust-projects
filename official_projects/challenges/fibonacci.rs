// Generate the nth Fibonacci number
fn main() {
    let n = 5;
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