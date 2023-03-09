# Project: Guessing Game

[Source code](guessing_game/src/main.rs)

To bring the io input/output library into scope

```rust
use std::io;
```

The io library comes from the standard library, known as ```std```.

**Prelude**:

- The set of items which are defined in the standard library that Rust brings into the scope of every program.

- If the required type is not in the prelude, we have to bring that type into scope explicitly with a ```use``` statement.

**Mutable variable**:

```rust
let mut guess = String::new();
```

- ```String```: a type provided by standard library that is a growable, UTF-8 encoded bit of text
- ```::``` indicates that ```new```is an associated function(a function implemented on a type) of the ```String```type(in this case).
- ```new``function creates a new, empty string here.
- ```String::new``` function that returns a new instance of ```String```

So, the whole code line, creating a mutable variable that is currently bound to a new, empty instance of a String.

**Receiving User input**:

```rust
io::stdin()
    .read_line(&mut guess)
```

- ```std::io::stdin()``` returns an instance of ```std::io::Stdin```, a type that represents a handle to the standard input from command line.
- ```.read_line(&mut guess)```: calls the ```read_line```method on the standard input handle to get input from the user.
- ```&mut guess``` is for telling to what variable the user input should be stored.
- ```&``` indicates that this argument is a *reference*. A way to let multiple parts of the code access once piece of data, avoiding the need to copy data into memory multiple times.
- *references* are immutable by default. So, to make it mutable we have to write ```&mut guess```rather than ```&guess```.
- The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents)

**Handling Potential Failure with Result**:

```rust
.expect("Failed to read line");
```

- ```read_line``` puts whatever the inputs to string we pass to it, but it also returns a ```Result``` value.
- ```Result```is an enum with two possible variants ```ok```and ```err```(Enum(enumeration) which can have multiple possible states called variants).
- ```ok```: indicates operation was successful.
- ```err```: indicates operation failed.
- An instance of ```Result```has an ```expect``` method.
- If the instance of ```Result```is an ```Err``` value, ```expect```will cause the program to crash and display the message we passed using ```expect```.
- If the instance of ```Result```is an ```ok```value, ```expect``` will take the return value that ```ok```is holding and return just that value(in this case, the number of bytes in the user's input) for further use.

**Printing Values with println! Placeholders**:

```rust
println!("You guessed: {guess}");
```

- ```{}```: placeholder

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

Output:

```bash
x = 5 and y = 12
```

**Generating a Secret Number**:

- *Crate* is a collection of Rust source code files.
- ```rand``` [crate](https://crates.io/crates/rand) is a *library crate* which cannot be executed on its own, instead to be used in other programs.

*Cargo.toml*

```bash
[dependencies]
rand = "0.8.5"
```

After adding the above build project using ```cargo build```.

**Ensuring Reproducible Builds with the Cargo.lock File**:

- When we build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the *Cargo.lock* file.
- When we build project in future, Cargo will see that the Cargo.lock file exists and will use the versions specified there. 
- This lets us have a reproducible build automatically.

**Updating a Crate to Get a New Version**:

```cargo update```will ignore *Cargo.lock* file and use all the latest versions specified in *Cargo.toml* and write all those versions to *Cargo.lock* file.

**Generating a Random Number**:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

- ```use rand::Rng;```
  - ```Rng``` trait defines methods that random number generators implement.
  - This train must in the scope to use those methods.
- ```rand::thread_rng()``` gives random number generator.
- Then call the ```gen_range()``` method on that generator. This method is defined by ```Rng``` trait.  

**Comparing the guess to the secret number**:

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- The Ordering type is another enum and has the variants ```Less```, ```Greater```, and ```Equal```.
- ```cmp``` method compare two values and returns a variant of the ```Ordering``` enum.
- ```match```expression is used to decide what to do next based on which variant of ```Ordering``` was returned from the call to cmp with the values in guess and secret_number.
- A ```match``` expression is made up of *arms*. An arm consists of:
  - a pattern to match against
  - and the code that should be run if the value given to match fits that arm’s pattern.
  - The ```match``` expression ends after the first successful match, so it won’t look at the last arm in this scenario.

Converting variable to another type by *shadowing*:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

- The ```parse``` method on strings converts a string to another type.
- ```parse``` method returns a ```Result``` type(enum). Upon error it will give ```err``` variant cause the program exit with the error message given using ```expect```. Upon success it will return ```ok``` variant of Result and expect will return the number.

**Allowing Multiple Guesses with Looping With Quiting after correct input**:

```rust
println!("The secret number is: {secret_number}");

loop {
    println!("Please input your guess.");

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
```

**Handling Invalid Input**:

```rust
// --snip--

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {guess}");

// --snip--
```

If ```pars``` is able to successfully turn the string into a number:

- it will return an ```Ok``` value that contains the resultant number.
- That ```Ok``` value will match the first arm's pattern
- and the ```match``` expression will just return the num value that parse produced and put inside the Ok value.

If ```parse``` is not able to turn the string into a number:

- It will return an```Err``` value that contains more information about the error.
- underscore(```_```) is a`catchall value, represents what ever value inside ```Err```.
