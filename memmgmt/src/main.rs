
#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person
}


impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }
}


fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = x; // copied

    let v = vec![1, 2, 3];
    let v2 = v; // transferred ownership

    // println!("{}", v[0]); // error, v is no longer valid
    // v no longer accessible
    println!("{}", v2[0]);

    let mut a = 6;

    // mutable reference
    let b = &mut a;
    println!("{}", *b);
    println!("{}", a);
    // error because a has been returned above
    // b += 2

    // scope
    {
        let c = &mut a;
        *c += 2;
        println!("{}", *c);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);

        // Memory guarantee will not allow push operation on the same vector that is borrowed and pushed to
        // v.push(6);

    }

    let p1 = Person { name: String::from("Alex1") };
    let d1 = Dog { name: String::from("Fido"), owner: &p1 };

    let mut a: &String;
    {
        let p2 = Person { name: String::from("Alex") };
        // a = p2.get_name();
        // a = p2.get_name();  // borrowed var does not live long enough
        a = p1.get_name();
    }

    println!("{}", a);
}


// Rust idea - one variable has one memory address or pointer

// Only one variable can own a piece of memory at a time
// For primitive types, the value is stored on the stack
// For complex types, the value is stored on the heap
// For primitive types, copying data is cheap because we know the size of the data at compile time
// for complex types, ownership is transferred when we assign a value to another variable