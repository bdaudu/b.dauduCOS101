use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();


    for _c in 1..151{
        println!("Enter your name: ");
    io::stdin().read_line(&mut input1);
    let name = input1.trim();

    println!("Enter your e-mail: ");
    io::stdin().read_line(&mut input2);
    let email = input2.trim();

    println!("Enter your department: ");
    io::stdin().read_line(&mut input3);
    let d = input3.trim();

    println!("What is your state of origin?: ");
    io::stdin().read_line(&mut input4);
    let soo = input4.trim();

    println!("Are you a class rep? : ");
    io::stdin().read_line(&mut input5);
    let class_rep = input5.trim();

    println!("Are you in 100 level? ");
    io::stdin().read_line(&mut input6);
    let level = input6.trim();

    println!("Enter your CGPA: ");
    io::stdin().read_line(&mut input7);
    let cgpa:f32 = input7.trim().parse().expect("Not a valid numer");

    if class_rep == "yes" || class_rep == "Yes" && level == "no" || level == "No" && cgpa > 4.0{
        println!("\n{} \n{} \n{} \n{} \nYou can vote.",name,email,d,soo);
    }
    else{
        println!("Sorry, you are not eligible to vote!", );
    }
    }
}
