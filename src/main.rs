#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // Strings scalar slices
    let cat: &str = "Fluffy"; // & represents memoryaddress
    let cat: &'static str = "Fluff";
    let mut dog = String::from("Doge");
    let formatted = format!("{} is a {}", "Dog", dog);
    println!("{} {}", dog, cat);
    let mut doggr = "wow";

    dog.push(' '); // has to be mutable to push
    dog.push_str("Dogeee");

    const URL: &str = "Gooooogle OOGLE";

    println!("{}", dog);

    say_hi("miha");

    for i in 0..6 {
        say_hi("doggo");
        say_hi_mutable(&mut doggr);
    }

    println!("{}", dog)
}

// intorduction to functions
fn say_hi(name: &str) {
    // pass by value
    println!("hello there {}", name)
}

fn say_hi_mutable(name: &mut &str) -> String {
    // pass by value
    println!("hello there {}", name);
    *name = "I changed it";
    return format!("{}", name);
}
