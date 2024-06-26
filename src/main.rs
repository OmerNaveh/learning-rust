#![allow(unused)]

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;
mod blog;
use blog::blog_site::run;
use std::fs::File;

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
impl Complex {
    fn is_equal(&self, other: &Complex) -> bool {
        self.real == other.real && self.imag == other.imag
    }
    fn simply_zero() -> Complex {
        Complex {
            real: 0.0,
            imag: 0.0,
        }
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


fn main() {
    // guessing_game();
    // blog::blog_site::run();
    // cli_app();

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn generic_get_largest<T: PartialOrd + Copy>(list :Vec<T>)->T{
    let mut greatest = list[0];
    for &item in list.iter(){
        if item > greatest{
            greatest = item;
        }
    }
    greatest
}
fn populate_vector(v: &mut Vec<i32>,size:i32) {
    for i in 1..size {
        v.push(i);
    }
}
fn binary_search(v: &Vec<i32>, target:i32)-> Option<usize>{
    let mut low = 0;
    let mut high = v.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        match v[mid].cmp(&target) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }
    None

}
fn vectors (){
    let mut v:Vec<i32> = Vec::new();
    populate_vector(&mut v,101);

    let target = 50;
    match binary_search(&v, target){
        Some(index) => println!("{} found at index {}", target, index),
        None => println!("{} not found", target),
    }
}
fn hashmaps(){
    let s = String::from("Hello world you world");
    let mut map:HashMap<&str, u32> = HashMap::new();

    for word in s.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
fn fs_ops(){
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(mut fc) => {
                    fc.write(b"Hello World");
                     fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
fn guessing_game() {
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_number);
    loop {
        println!("Guess the number!");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(u32_num) => u32_num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

fn cli_app(){
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let content = read_file_contents(&config.filename).unwrap_or_else(|err|{
        println!("Problem reading file: {}", err);
        std::process::exit(1);
    });

    println!("File contents: {}", content);
}
struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn new(args: Vec<String>) ->Result<Config,String>{
        if args.len() < 3 {
            return Err("Not enough arguments".to_string());
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query,filename})
    }
}

fn read_file_contents(filename: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    Ok(content)

}
