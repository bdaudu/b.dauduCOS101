use std::io;

fn main() {
    let mut input1 = String::new();
    println!("Enter in your name:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");
}
