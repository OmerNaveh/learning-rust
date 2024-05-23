#![allow(unused)]
use std::fmt;
use std::io;

struct MyType {
    name: String,
    age: u32,
}
impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyType {{ name: {}, age: {} }}", self.name, self.age)
    }
}

impl fmt::Debug for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug {{ name: {}, age: {} }}", self.name, self.age)
    }
}

struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Complex {{real: {real} , imag: {imag}}}",
            real = self.real,
            imag = self.imag
        )
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?;
        }
        write!(f, "]")
    }
}

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Day {
    fn is_weekend(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => true,
            _ => false,
        }
    }
    fn morning(&self) -> String {
        match self {
            Day::Monday => "Monday morning".to_string(),
            Day::Tuesday => "Tuesday morning".to_string(),
            Day::Wednesday => "Wednesday morning".to_string(),
            Day::Thursday => "Thursday morning".to_string(),
            Day::Friday => "Friday morning".to_string(),
            Day::Saturday => "Saturday morning".to_string(),
            Day::Sunday => "Sunday morning".to_string(),
            _ => "Invalid day".to_string(),
        }
    }
}

fn print_day(day: Day) {
    println!("Is {} a weekend? {}", day.morning(), day.is_weekend());
}

fn main() {
    // println!("What's your name?");
    // let mut name = String::new();
    // io::stdin().read_line(&mut name).expect("Failed to read line");
    // let name = name.trim_end();
    // println!("Hello, {}!", name);
   
    let day = Day::Monday;
    print_day(day);
    let day = Day::Saturday;
    print_day(day);

}
