/*
// Returning Values from Loops
fn main() {
    let mut counter = 0;
  
    let result = loop {
        counter += 1;
  
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}*/

/* 
// Loop Labels to Disambiguate Between Multiple Loops
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
*/

// Conditional Loops with while
/*fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}*/

/*
// looping through collection using while
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}*/

// looping though collection using for
/*fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The values is: {element}");
    }
}*/

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}