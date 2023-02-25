fn main() {
    // Example showing variable immutability by default
    variable_immutablity_by_default();

    // Example showing making the variable mutable
    variable_mutability();

    // Shadowing
    shadowing();
    
}

fn variable_immutablity_by_default() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}")
}

fn variable_mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
