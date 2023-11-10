// Program to calculate how fast a car is travelling 

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter the Distance covered by the car (In Miles): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut d:f32 = input1.trim().parse().expect("Not a valid number");
    //change to Kilometres
    d = d * 1.609344; 
    println!("The distance in Kilometres is: {}",d);
    

    println!("Enter the time taken to cover this distance: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");    
     
    // Finding speed
    let unit = "Kilometres/Hour"; 
    let speed = d / t;
    println!("the speed of the car is: {} {}",speed,unit);
}
