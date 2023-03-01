# Rust Programming Language Book Notes

Based on the contents of the **Book**: [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)

## Installation

**On Mac**:

```rustup```: a command line tool for managing Rust versions and associated tools.

```bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

To Install linker, which is a program that Rust uses to join its compiled outputs into one file.

```bash
xcode-select --install
```

To check version:

```bash
rustc --version
```

To update Rust:

```bash
rustup update
```

To open the local documentation in your browser:

```bash
rustup doc
```

To run a rust script:

```bash
rustc script.rs
```

## Cargo

Cargo is Rust’s build system and package manager. Cargo handles tasks such as:

- Building the code
- Downloading the libraries the code depends on, and building those libraries

To check Cargo version:

```bash
cargo --version
```

### Project creation with Cargo

```bash
cargo new hello_cargo
```

- creates a new directory and project called *hello_cargo*.
- Under the directory it will create:
  - *Cargo.toml* file  (TOML: Tom's Obvious, Minimal Language is Cargo's configuration format)
    - ```[package]``` heading indicates that the following statements are configuring a package.
    - Next thee lines(name, version, edition) set the config information Cargo needs to compile your program.
    - ```[dependencies]```section to list project's dependencies.
    - *crates*: packages of code are referred to as crates in Rust.
  - *src* directory with a main.rs file inside.
- Also will initialize a new Git repository along with a *.gitignore* file. Git files won’t be generated if run *cargo new* within an existing Git repository.

### Build and Run Cargo project

Build project with following command inside project directory:

```bash
cargo build
```

- If building for first time, creates a file named *Cargo.lock* at the top level to keep track of exact version of dependencies in the project.
- Creates an executable file in *target/debug/project_name* rather than in the current directory.
- Since the default build is a **debug build**, Cargo puts the binary in a directory named *debug*.

To run the executable:

```bash
./target/debug/project_name
```

To do build and run in single command:

```bash
cargo run
```

Command to check the code to make sure it compiles but does not produce an executable,

 ```bash
 cargo check
 ```

- ```cargo check```faster than ```cargo build```
- So, can be used to continually checking the project to make sure that it still compiling. 

### Building for Release

When ready for release, to compile the project with optimizations:

```bash
cargo build --release
```

- create an executable in ```target/release``` instead of ```target/debug```.
- Optimizations makes the Rust code run faster but takes more time to compile.
- Because of this, there are two different profiles:
  1. for development(without ```--release```): when we want to rebuild quickly and often.
  2. for building the final program(with ```--release```) to make the runs as fast as possible.
- For bench marking, run ```cargo build --release```and bench mark with executable in ```target/release```.

## Common programming concepts

### Variables and Mutability

- Variables are immutable by default which make us write code in a way that takes advantage of the safety and easy concurrency the language offers.
- But, have the option to make variable mutable.

Erroneous code example:

```rust
fn main() {
    let x = 5;
    x = 6; // error, reassignment of immutable variable
    println!("The value of x is: {x}")
}
```

Throws error: ```cannot assign twice to immutable variable x```

- Rust compiler guarantees that when we stat that a value won't change, it really won't change. So, we don't need to track of it ourselves.

- To make the variable mutable use `mut` eg:

```rust
fn main() {
    let mut x = 5;
    x = 6;
    println!("The value of x is: {x}") 
}
```

Output will be: ```The value of x is: 6```

#### Constants

- Similar to immutable by defaults but cannot use ```mut``` with them to make it mutable.
- Declare with ```const``` keyword and type of value must be annotated.
- Can be declared in any scope. eg. using global scope make them usable for many parts of the code.
- Constant must set only to a constant expression not the result of a value computed only at runtime.
- Rust’s naming convention for constants is to use all uppercase with underscores between words.

eg:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

#### Shadowing

We can shadow a variable by using the same variable's name and repeating the use of the ```let``keyword.

eg:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

output:

```bash
The value of x in the inner scope is: 12
The value of x is: 6
```

- By using ```let```, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
- With ```let```we are effectively creating a new variable. So, we can change the type of the value but reuse same name.
eg:

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

But, with ```mut``` it's not possible to change variable type.

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
}
```

Will throw compile-time error: ```error[E0308]: mismatched types```

### Data Types

Rust is a statically typed language: means it must know the types of all variables at compile time.

Compiler can usaually infer the datatype based on value. But in case multiple types are possible, we must add a type annotation.

eg:

```rust
let guess: u32 = "42".parse().expect("Nota a number!");
```

There are two kind of data types:

- Scalar
- Compound

#### Scalar

- Represents a single value
- 4 primary scalar types: integers, floating-point numbers, Booleans and Characters

##### Integer

*Integer types in Rust*:
<figure>
  <img src="images/integer_type_in_rust.png" alt="Integer types in rust" width=60% height=60%>
</figure>

*Integer Literals in Rust*:
<figure>
  <img src="images/integer_literals_in_rust.png" alt="Integer literals in rust" width=60% height=60%>
</figure>

- When integer overflow offers(eg: change value of a u8 variable to 256), can result in one of two behaviours:
  - *Panicking*: when compiling in debug mode, Rust includes checks for integer overflow that cause program to panic at run time.
  - *Two's complement wrapping*: when compiling in release mode with ```release```flag, Rust does not include checks for integer oveflow that cause panics. Instead if overflow happens, Rust performs *two's complement wrapping*. eg: in case of a u8, the value 256 becomes 0, the value 257 becomes 1 and so on.
- To handle possibility of overflow, following methods can be used:
  - ```wrapping_*```. eg: ```wrapping_add```
  - Return ```none```with ```checked_*```methods.
  - Return boolean(overflow or not) with ```overflowing_*``` methods.
  - Saturate at the value's min or max values with ```saturating_*```.

##### Floating-Poing Types.

- Two types: ```f32```and ```f64```with default type is ```f64```.
- All floating-point types are signed.

```rust
fn main() {
  let x = 2.0: // f64
  let y: f32 = 3.0; // f32
```

##### Numeric Operations

```rust
fn main() {
  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;
}
```

##### Boolean Type

```rust
fn main() {
  let t = true;

  let f: bool = false; // with explicit type annotation
}
```

##### Char type

```rust
fn main() {
  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';
}
```

- specify ```char```literals with single quotes as opposed to string literals which use double quotes.
- ```char```type is four bytes in size.

#### Compound

- Compound types can group multiple values into one type.
- Rust has two primitive compound types:
  - *tuples*
  - *arrays*

##### Tuple type

- Grouping number of values with a variety of types.
- Have fixed length

eg:

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  let (x, y, z) = tup; // pattern matching to destructure a tuple value

  println!("The value of y is: {y}");

  // access tuple element using index.

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;
}
```

- A tuple without any values is called ```unit```. Its value and type are written ```()``` represents empty value and empty return type.

##### Array Type

- Collection of multiple values of same type.
- Have fixed length

```rust
fn main() {
  let a: [i32; 5] = [1, 2, 3, 4, 5];

  let first = a[0];
  let second = a[1];
}
```

Initialize array with same values:

```rust
let a = [3; 5]; // 5 elements with all values initialize to 3
```

- When we attempt to access an element using indexing, Rust will check that the index specified is less than the array length. If the index is greater than or equal to the length, Rust will panic.
- In that way Rust provides memory safety by preventing accessing of invalid memory.

### Functions

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

#### Parameters

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

In function signatures, we must declare the type of each parameter.

Multiple parameters:

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

#### Statements and Expressions

**Statements** do not return values.

eg:

```rust
fn main() {
    let y = 6;
}
```

```rust
fn main() {
    let x = (let y = 6);
}
```

Will get error as statement cannot assign to another variable(here ```let```). ```let y = 6``` statement does not return a value, so there isn't anything for x to bind to.

Contrary to this, **Expressions** evaluate to a value. eg: math operation 5 + 6 is an expression which evaluates to 11. E

- Expressions can be part of statement.
- Calling a function is an expression.
- Calling a macro is an expression.
- A new scope block created with curly brackets is an expression as given in example below.

```rust
fn main() {
  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is: {y}");
}
```

Expressions do not include ending semicolons. eg: ```x + 1``` above. If we add a semicolon to the end of an expression, it will turn it into a statement, and it will then not return a value.

#### Functions with Return Values

- The return value of the function is synonymous with the value of the final expression in the block of the body of a function. But can also reutrn early using ```return``` keyword.

eg: 

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

### Comments

```rust
fn main() {
  // hello, world
}
```

### Control Flow

#### if Expressions

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

- Condition must be always ```bool``` type.
- The block of code in the if condition block is called *arm*.

##### Handling Multiple Conditions with else if

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

##### Using if in a let Statement

Because if is an expression, it's possible to use it on the right side of a let statement to assign the outcome to a variable.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

- Values in the both arm should be of same type. It's because,
  - Blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.
  - In this case, the value of the whole if expression depends on which block of code executes.
  - During the compile time, Rust need to know what type the ```number``` variable is.

#### Repetition with Loops  

Rust has three kinds of loops: ```loop```, ```while```, and ```for```.

##### Repeating Code with loop

```loop``` execute its body to run again and again until we stop the program manually.

```rust
fn main() {
  loop {
    println!("again!");
  }
}
```

##### Returning Values from Loops

```rust
fn main() {
    let mut counter = 0;
  
    let result = loop {
        counter += 1;
  
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
  }
```

Output:

```bash
The result is 20
```

##### Loop Labels to Disambiguate Between Multiple Loops

If there are multiple loops *loop lebel* helps to specify the ```break``` or ```contintue``` apply to which loop.

eg:

```rust
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
```

Output:

```bash
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

##### Conditional Loops with while

eg:

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

##### Looping Through a Collection with for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}
``` 

- This code would panic if we forgot to update the while condition after re-initializing array with less number of elements for example.
- It is also slow as compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every loop iteration.
- More concise way is using ```for``` loop.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The values is: {element}");
    }
}
```

Modification of aforementioned while loop to make it more "Rustian":

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
