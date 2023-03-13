# Understanding Ownership

Ownership enables Rust to make memory safety guarantees without needing a garbage collector.

## What is ownership

Ownership is a set of rules that govern how a Rust program manages memory.

- In Rust memory memory is managed through a system of ownership with a set of rules that the compiler checks.
- If any of the rules are violated, the program won't compile.
- No features of ownership will slow down the program when it's running.

### Stack and Heap

- Both the stack and the heap are parts of memory available to your code to use at runtime.
- **Stack**:
  - Last In First Out(LIFO) data structure.
  - All data stored on the stack must have a known, fixed size.
  - Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
- **Heap**:
  - The heap is less organized.
  - 

## References and Borrowing

## The Slice Type