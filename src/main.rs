#![allow(unused)]

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::io;
mod blog;
use blog::blog_site::run;

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

    // let mut v:Vec<i32> = Vec::new();
    // populate_vector(&mut v,101);

    // let target = 50;
    // match binary_search(&v, target){
    //     Some(index) => println!("{} found at index {}", target, index),
    //     None => println!("{} not found", target),
    // }

    let s = String::from("Hello world you world");
    let mut map:HashMap<&str, u32> = HashMap::new();

    for word in s.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

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
fn print_str(s: &String) {
    println!("{}", s);
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
