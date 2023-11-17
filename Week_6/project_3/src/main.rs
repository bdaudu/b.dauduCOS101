use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    loop {
            println!("This is where you can get any multiplication table \nWhich number should be used to multiply?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let n:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Last number in the mulplication table plus 1: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let l:i32 = input2.trim().parse().expect("Not a valid number");

    for _m in 1..l{
        let ans:i32 = n * _m;
        println!("{} x {} = {}",n,_m,ans);
    }
  }

}
