fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = x; // copied

    let v = vec![1, 2, 3];
    let v2 = v; // transferred ownership

    // println!("{}", v[0]); // error, v is no longer valid
    // v no longer accessible
    println!("{}", v2[0]);

}


// Rust idea - one variable has one memory address or pointer

// Only one variable can own a piece of memory at a time
// For primitive types, the value is stored on the stack
// For complex types, the value is stored on the heap
// For primitive types, copying data is cheap because we know the size of the data at compile time
// for complex types, ownership is transferred when we assign a value to another variable