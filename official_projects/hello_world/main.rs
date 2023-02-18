// This is the main function. The main function is special: it is always the first code that runs in every executable Rust program.
fn main() {
    // println! is a macro that prints text to the console. f it had called a function instead, it would be entered as println (without the !)
    println!("Hello World!"); // Rust style is to indent with four spaces, not a tab.
}

// Compile the program using rustc command which outputs a binary executable.
// To run: ./1_hello_world

// Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.