#[allow(unused_variables)]
#[allow(unused_assignments)]
mod player;

use crate::archive::arch::arch_file as arc;
use rand::Rng;

mod archive;

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

    println!("{}", dog);

    say_hi("miha");

    for i in 0..6 {
        say_hi("doggo");
        say_hi_mutable(&mut doggr);
    }

    println!("{}", dog);

    player::play_movie("dsd.mp4");
    arc("ssss.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();

    let primes = [1, 2, 3, 5, 2]; // Arrays cannot be resized and they are all the same type, however they can be modified
    let mut numbers = [0; 15]; // [0, 0, 0 , 0 ... 0] len 15

    const DEFAULT_NUM: i64 = 100;
    let mut numbers = [DEFAULT_NUM; 15];

    numbers[3] = 5; // needs to be mut

    println!("{:?}", numbers);

    println!("{}", a);

    for number in numbers.iter() {
        println!("{}", number)
    }

    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 4, 8, 9, 6];

    primes.push(7);
    primes.remove(2);

    /* Slices - pointers to certain amount of memory */
    println!("{:?}", &numbers[1..3]); // second to third element

    println!("{:?}", primes);

    let mut colors = ["red", "pink", "gray", "blue"];
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);
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

// param - mutable array (through function) of strings
fn update_colors(colors: &mut [&str]) {
    colors[0] = "yellow";
    colors[1] = "orange"
}

mod clean {
    pub fn perform() {
        // pub exposes it outside the module
        println!("cleaning hdd");
    }
}
