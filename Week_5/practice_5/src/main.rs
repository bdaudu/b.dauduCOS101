// Rust program to read the height of a persn
// and then print if person is tall, short,
// or average height person

use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height (in centimeters): ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0 
    {
            println!("You're an average height person");
    } 
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are short");
    }
    else 
    {
        println!("Are you a human being?");
    }
    }

