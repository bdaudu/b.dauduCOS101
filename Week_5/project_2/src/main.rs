use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Do you have work experience?: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience = input1.trim();

    println!("\nEnter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32= input2.trim().parse().expect("Not a valid number");

    if experience == "yes" && age >= 40{
        println!("You have an incentive of 1,560,000 Naira");
    }
    else if experience == "yes" && age >=30 && age <40{
        println!("You have an incentive of 1,480,000 Naira");
    }
    else if experience == "yes" && age < 30{
        println!("You have an incentive of 1,300,000 Naira");
    }
    else{
        println!("You're an inexperienced employee");
    }
}
