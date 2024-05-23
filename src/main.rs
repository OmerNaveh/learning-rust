#![allow(unused)]

use std::fmt;
use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;


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
//    guessing_game();


}


fn guessing_game(){
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}",random_number);
    loop{
        println!("Guess the number!");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u32 = match guess.trim().parse(){
            Ok(u32)=> u32,
            Err(_)=> continue,
        };

        match guess.cmp(&random_number){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
       
    }
}