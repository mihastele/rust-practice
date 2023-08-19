fn main() {
    println!("Hello, world!");
}


// Rust idea - one variable has one memory address or pointer

// Only one variable can own a piece of memory at a time
// For primitive types, the value is stored on the stack
// For complex types, the value is stored on the heap
// For primitive types, copying data is cheap because we know the size of the data at compile time
// for complex types, ownership is transferred when we assign a value to another variable