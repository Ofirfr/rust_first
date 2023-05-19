
use std::{io, string};
use rand::Rng;
fn main() {
    println!("Please enter your age:");
    let mut string_age = String::new();
    io::stdin().read_line(&mut string_age).expect("-1");
    let age: i16;
    let ageResult = string_age.trim().parse::<u16>();
    match ageResult {
        Ok(ageNumber) => age = ageNumber as i16,
        Err(msg) => {
            println!("{}", msg);
            age = -1;
        },
    }
    if (age>18) && (age<100) {
        println!("You can vote, as you are {}", age);
    }
    else{
        println!("As a {} year old, you cant vote.", age);
    }
    println!("Hello, world!");
}