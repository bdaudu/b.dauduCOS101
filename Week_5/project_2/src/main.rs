use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your age: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let age:i32= input.trim().parse().expect("Not a valid number");

    if age >= 40{
        println!("You're an experienced employee with an incentive of 1,560,000 Naira");
    }
    else if age >=30 && age <40{
        println!("You're an experienced employee with an incentive of 1,480,000 Naira");
    }
    else if age < 30{
        println!("You're an experienced employee with an incentive of 1,300,000 Naira");
    }
    else{
        println!("You're an inexperienced employee");
    }
}
