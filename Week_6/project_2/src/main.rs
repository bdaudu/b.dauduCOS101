use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    for _x in 1..501{
    println!("Your name:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name = input1.trim();

    println!("How many number of papers have you published? ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let nopp:i32 = input2.trim().parse().expect("Not a valid number");
    if nopp >= 3 && nopp <= 5{
        println!("Your incentive is 500,000 naira \n{}",name);
    }
    else if nopp > 5 && nopp < 10{
        println!("Your incentive is 800,000 naira \n{}",name);
    }
    else if nopp >= 10{
        println!("Your incentive is 1,000,000 naira \n{}",name);
    }
    else if nopp < 3{
        println!("Your incentive is 100,000 naira \n{}",name);
    }
    else{
        println!("Did you publish any papers?");
    }
    }
}
