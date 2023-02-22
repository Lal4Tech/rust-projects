fn main() {
    // Example showing variable immutability by default
    /*let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");*/

    // Example showing making the variable mutable
    /*let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");*/

    // Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    
}
