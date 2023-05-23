use std::ops::Add;
use std::path::Component::ParentDir;

struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}


trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) { println!("Hello world!") }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\" Hello World!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome }
    }

    fn language(&self) -> &str {
        "Java 17"
    }

    fn say_hello(&self) {
        println!("System.out.println(\" Hello World!\");");
    }
}


trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species);
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark());
}


// WE CANNOT RETURN A TRAIT DUE TO MEMORY GUARANTEE
struct Doge {}

struct Cate {}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Doge {
    fn make_noise(&self) -> &'static str {
        "WOOF"
    }
}

impl Animal for Cate {
    fn make_noise(&self) -> &'static str {
        "MEOW"
    }
}

// dynamic
fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Doge {})
    } else {
        Box::new(Cate {})
    }
}


trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// static duplicateable

trait Duplicateable {
    fn dupl(&self) -> String;
}

impl Duplicateable for String {
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicateable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate<T: Duplicateable>(x: T) {
    println!("{}", x.dupl());
}

fn duplicate_dyn(x: &dyn Duplicateable) {
    println!("{}", x.dupl());
}

fn main() {
    let r = RustDev { awesome: true };
    let r2 = RustDev::new(true);
    let j = JavaDev::new(true);

    j.say_hello();
    println!("{}", j.language());
    r.say_hello();
    println!("{}", r.language());

    let dog = Dog { species: "retriever" };
    let cat = Cat { color: "orange" };

    bark_it(dog);
    // can't, not availbale for cat
    // bark_it(cat);

    println!("Animal dude says {}", get_animal(0.5).make_noise());
    println!("Animal dude says {}", get_animal(2.0).make_noise());

    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum());

    // let b = vec![1.0, 2.0, 3.0, 4.0 ,5.0];
    // println!("sum = {}", a.sum());

    let p1 = Point { x: 1.3, y: 4.6 };
    let p2 = Point { x: 3.7, y: 1.4 };
    let p3 = p1 + p2;

    println!("{:?}", p3);

    let a: i32 = 42;
    let b: String = "Hi mom!".to_string();

    duplicate(a);
    duplicate(b);

    duplicate_dyn(&a);
}
