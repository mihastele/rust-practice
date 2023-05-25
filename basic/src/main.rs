use crate::Colors::Red;
use crate::EnumWithTypes::Name;
use crate::Suit::{Diamond, Heart};


macro_rules! my_macro {
    () => (println!("first macro"))
}

macro_rules! name {
    ($name: expr) => ( println!("hey {}", $name) )
}

macro_rules! name_name {
    ($name: expr, $name2: expr) => ( println!("hey {} and {}", $name, $name2) )
}

macro_rules! name1 {
    ($($name: expr),*) => ( $(println!("hey {}", $name);)* )
}

macro_rules! xy {
    (x => $x: expr) => ( println!("X is {}", $x) );
    (y => $x: expr) => ( println!("Y is {}", $x) );
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name))
        }
    }
}

fn main() {
    println!("Hello, world!");

    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];

    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);

    let person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);
    println!("{:?}", person.0);
    println!("{:?}", person.1);


    let mut mutperson: (&str, i64, bool) = ("John", 27, true);
    mutperson.0 = "Mike";
    println!("{:?}", mutperson);
    println!("{:?}", mutperson.0);
    println!("{:?}", mutperson.1);

    let (name, age, employment) = person;
    println!("name: {}, age: {}, employed: {}", name, age, employment);


    let emp1 = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35,
    };

    print!("{:?}", emp1);
    println!("{}", emp1.name);
    println!("{}", emp1.fn_details());
    println!("{}", Employee::static_fn_detail());

    let my_color = Colors::Red;

    let myc = Red;
    println!("{:?}", my_color);

    let person = Name(String::from("Alex"));
    println!("{:?}", person);

    let p1: Point<i32> = Point { x: 6, y: 8 };
    let p2: Point<f64> = Point { x: 6.1, y: 8.4 };

    println!("{:?}", p1);
    println!("{:?}", p2);

    let c2 = ColorsGeneric::Red("#f00");
    let c3 = ColorsGeneric::Red(255);

    let p3: Point2Gen<i32, f64> = Point2Gen { x: 32, y: 6.6 };

    print_choice(Heart);
    print_choice(Diamond);


    for i in 0..15 {
        println!("{} I have {} oranges", i, get_oranges(i))
    }


    let point = (0, 5);

// tuple pattern matching
    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("({}, {})", x, y)
    }

    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i * i)
    }

    let pets = ["cat", "dog", "cat", "chihuahua", "gg", "bear"];

    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("{} barks too much", pet);
            continue;
        }
        if pet == &"gg" {
            println!("Game over, report!");
            break;
        }
        println!("I love my {}", pet);
    }
    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, square);
    }

    get_squares(333);
    get_cubes(333);

    // closures
    |a: i32, b: i32| -> i32 {
        println!("sum {}", a + b);
        // return a + b;
        a + b
    };

    let closure_can_be_a_variable = |a: i32, b: i32| println!("sum {}", a + b);
    closure_can_be_a_variable(3, 2);

    let sqr = |a: i32| a * a;
    apply(sqr, 5);
    apply(divide_by_2, 10);

    // Higher order functions

    // bloated code with non HOF
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit { break; } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("Sum without HOF is {}", sum);

    // With HOF
    let sum2 = (0..).map(|x: i32| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("Sum with HOF is {}", sum2);

    // macros
    my_macro!();
    name!("James");
    name1!("Man", "Mannen", "Kvinna", "Kvinnan");
    xy!(x => 5);
    name_name!("a", "b");
    build_fn!(sd);
    sd()
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn divide_by_2(a: i32) -> i32 {
    a / 2
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Res {}", f(a));
}

fn get_squares(limit: i32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0} * {0} = {1}", x, x * x);
        x += 1;
    }
}

fn get_cubes(limit: i32) {
    let mut x = 1;
    loop { // while true
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break;
        }
    }
}

// .. exclusive end
// ..= inclusive end
fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "Unkown",
        _ => "Invalid"
    };

    println!("{:?}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => { println!("\u{2665}") }
        Spade => { println!("\u{2660}") }
        Club => { println!("\u{2663}") }
        Diaomond => { println!("\u{2666}") }
    }
}


// Pattern matching
fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "Lots of"
    };
}


#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum EnumWithTypes {
    Name(String),
    Surname(String),
    Age(u32),
}

// Derive for debug println!
#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}


#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2Gen<T, V> {
    x: T,
    y: V,
}

enum ColorsGeneric<T> {
    Red(T),
    Blue(T),
}

impl Employee {
    fn fn_details(&self) -> String {
        return format!("name: {}, age {}, company: {}", &self.name, &self.age, &self
            .company);
    }
    // static doesn't have a self reference
    fn static_fn_detail() -> String {
        String::from("Details of a person")
    }
}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
}