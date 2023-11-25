use std::io;

fn main() {

    let mut input1 = String::new();
    println!("How many siblings do you have?: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let nos:i32 = input1.trim().parse().expect("Invalid input");

  for _siblings in 1..nos+1{
    let mut input2 = String::new();
    println!("Oldest sibling's age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let s:i32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("What is your marital status?: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let ms:&str = &input3;

    let mut input4 = String::new();
    println!("Are you a student or a worker?: ");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let stu_worker:&str = &input4;

    let mut input5 = String::new();
    println!("Have you written WAEC?: ");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let waec:&str = &input5;

    if s > 18 && ms == "single" && stu_worker == "student" && stu_worker == "worker" && waec == "yes"{
    println!("{:#?}", uni_course());
    }
    else if s > 18 && ms == "married" && stu_worker == "student" && stu_worker == "worker" && stu_worker == "none" && waec == "yes"{
        println!("{:#?}", offspring_city());
    }
    else if s < 18 && ms == "single" && stu_worker == "student" && waec == "yes"{
        println!("{:#?}", secondary());
    }
    else {
        println!("ERROR!");
    }
  }

}

fn uni_course(){
    let mut input6 = String::new();
    println!("Which university did you attend?: ");
    io::stdin().read_line(&mut input6).expect("Failed to read input");

    let mut input7 = String::new();
    println!("Which course did you study? ");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
}

fn offspring_city(){
    let mut input8 = String::new();
    println!("Do you have any children?: ");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let os:&str = &input8;

    let mut input9 = String::new();
    println!("Which city does your family live in? ");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
}

fn secondary(){
    let mut input10 = String::new();
    println!("What is the name of your secondary school?: ");
    io::stdin().read_line(&mut input10).expect("Failed to read input");

    let mut input11 = String::new();
    println!("Current class level?");
    io::stdin().read_line(&mut input11).expect("Failed to read input");
    let cl:i32 = input11.trim().parse().expect("Invalid input");
}
