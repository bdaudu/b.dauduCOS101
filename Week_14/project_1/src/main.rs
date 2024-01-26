use std::io;
use std::io::Read;

fn main() {
    for x in 0..50{
    println!("Welcome to the Globacom Ltd Database\nOwned by Mike Adenuga (Chairman/CEO)\n");
    println!("Are you an administrator, project manager, employee, customer or vendor?\n(Enter in lowercase)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let post = input1.trim();

    if post == "administrator" || post == "admin" {
        admin();
    }
    else if post == "project manager" {
       project_db();
    }
    else if post == "staff" || post == "employee" {
        employee();
    }
    else if post == "customer" {
        customer();
    }
    else if post == "vendor" {
        vendor();
    }
    else{
        println!("Please put in the correct post!");
    }
    }

}
fn admin(){
    println!("Access granted, administrator!\n");
        let mut file = std::fs::File::open("globacom_dbase.sql");
        let mut contents = String::new();
        file.expect("REASON").read_to_string(&mut contents).unwrap();
        print!("{}",contents);
}

fn project_db(){
    println!("Access granted, project manager!\n");
        let mut file = std::fs::File::open("project_tb.sql");
        let mut contents = String::new();
        file.expect("REASON").read_to_string(&mut contents).unwrap();
        print!("{}",contents);
}

fn employee(){
    println!("Access granted,employee!\n");
        let mut file = std::fs::File::open("staff_tb.sql");
        let mut contents = String::new();
        file.expect("REASON").read_to_string(&mut contents).unwrap();
        print!("{}",contents);
}

fn customer(){
        println!("Access granted, customer!\n");
        let mut file = std::fs::File::open("customer_tb.sql");
        let mut contents = String::new();
        file.expect("REASON").read_to_string(&mut contents).unwrap();
        print!("{}",contents);
}

fn vendor(){
    println!("Access granted,vendor!\n");
        let mut file = std::fs::File::open("dataplan_tb.sql");
        let mut contents = String::new();
        file.expect("REASON").read_to_string(&mut contents).unwrap();
        print!("{}",contents);
}