fn main() {
    let oa = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let a = vec!["No position","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let l = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let t = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];

    let mut input1 = String::new();
    println!("How many staff can use this program?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let s:i32 = input1.trim().parse().expect("Invalid input");

    for index in 0..s{
    let mut input2 = String::new();
    println!("What is your department?");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let dep = input2.trim();

    let mut input3 = String::new();
    println!("How many years of work experience do you have?");
    std::io::stdin().read_line(&mut input3).expect("Failed to read input");
    let yowp:f32 = input3.trim().parse().expect("Invalid input");

    if dep == "Office Administrator" && yowp >= 1.0 && yowp <= 2.0{
        println!("You hold the position APS 1-2:{}",oa[0]);
    }
    else if dep == "Office Administrator" && yowp >= 3.0 && yowp <= 5.0{
        println!("You hold the position APS 3-5:{}",oa[1]);
    }
     else if dep == "Office Administrator" && yowp >= 5.0 && yowp <= 8.0{
        println!("You hold the position APS 5-8:{}",oa[2]);
    }
     else if dep == "Office Administrator" && yowp >= 8.0 && yowp <= 10.0{
        println!("You hold the position EL1 8-10:{}",oa[3]);
    }
     else if dep == "Office Administrator" && yowp >= 10.0 && yowp <= 13.0{
        println!("You hold the position EL2 10-13:{}",oa[4]);
    }
     else if dep == "Office Administrator" && yowp >= 13.0{
        println!("You hold the position SES:{}",a[5]);
    }
    else if dep == "Academic" && yowp >= 1.0 && yowp <= 2.0{
        println!("You hold the position APS 1-2:{}",a[0]);
    }
    else if dep == "Academic" && yowp >= 3.0 && yowp <= 5.0{
        println!("You hold the position APS 3-5:{}",a[1]);
    }
    else if dep == "Academic" && yowp >= 5.0 && yowp <= 8.0{
        println!("You hold the position APS 5-8:{}",a[2]);
    }
    else if dep == "Academic" && yowp >= 8.0 && yowp <= 10.0{
        println!("You hold the position EL1 8-10:{}",a[3]);
    }
    else if dep == "Academic" && yowp >= 10.0 && yowp <= 13.0{
        println!("You hold the position EL2 10-13:{}",a[4]);
    }
    else if dep == "Academic" && yowp >= 13.0{
        println!("You hold the position SES:{}",a[5]);
    }
    else if dep == "Lawyer" && yowp >= 1.0 && yowp <= 2.0{
        println!("You hold the position APS 1-2:{}",l[0]);
    }
    else if dep == "Lawyer" && yowp >= 3.0 && yowp <= 5.0{
        println!("You hold the position APS 3-5:{}",l[1]);
    }
    else if dep == "Lawyer" && yowp >= 5.0 && yowp <= 8.0{
        println!("You hold the position APS 5-8:{}",l[2]);
    }
    else if dep == "Lawyer" && yowp >= 8.0 && yowp <= 10.0{
        println!("You hold the position EL1 8-10:{}",l[3]);
    }
    else if dep == "Lawyer" && yowp >= 10.0 && yowp <= 13.0{
        println!("You hold the position EL2 10-13:{}",l[4]);
    }
    else if dep == "Teacher" && yowp >= 13.0{
        println!("You hold the position SES:{}",l[5]);
    }
    else if dep == "Teacher" && yowp >= 1.0 && yowp <= 2.0{
        println!("You hold the position APS 1-2:{}",t[0]);
    }
    else if dep == "Teacher" && yowp >= 3.0 && yowp <= 5.0{
        println!("You hold the position APS 3-5:{}",t[1]);
    }
    else if dep == "Teacher" && yowp >= 5.0 && yowp <= 8.0{
        println!("You hold the position APS 5-8:{}",t[2]);
    }
    else if dep == "Teacher" && yowp >= 8.0 && yowp <= 10.0{
        println!("You hold the position EL1 8-10:{}",t[3]);
    }
    else if dep == "Teacher" && yowp >= 10.0 && yowp <= 13.0{
        println!("You hold the position EL2 10-13:{}",t[4]);
    }
    else if dep == "Teacher" && yowp >= 13.0{
        println!("You hold the position SES:{}",t[5]);
    }
    else{
        println!("Sorry! Error inputed information!");
    }
    }

}
