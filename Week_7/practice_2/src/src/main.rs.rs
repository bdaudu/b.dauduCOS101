use std::io;

fn main() {
    println!("Area of Trazepium: {}",a_o_f());
    println!("Area of Rhombus: {}",r_f());
    println!("Area of Parallelogram: {}",a_o_p());
    println!("Area of Cube: {}",a_o_c());
    println!("Volume of Cylinder: {}",v_o_c());
}

fn a_o_f()->f32{
    let mut input1 = String::new();
    println!("Area of a trapezium \nEnter base 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base1:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter base 2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base2:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let height:f32 = input3.trim().parse().expect("Invalid input");

    let aof = height/(2.0 * (base1 + base2));
    return aof;
}

fn r_f()->f32{
    let mut input4 = String::new();
    println!("\nArea of a rhombus \nEnter diagonal 1:");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let diagonal1:f32 = input4.trim().parse().expect("Invalid input");

    let mut input5 = String::new();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let diagonal2:f32 = input5.trim().parse().expect("Invalid input");

    let rf = 0.5 * diagonal1 * diagonal2;
    return rf;
}

fn a_o_p()->f32{
    let mut input6 = String::new();
    println!("\nArea of a parallelogram \nEnter base:");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let base:f32 = input6.trim().parse().expect("Invalid input");

    let mut input7 = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let altitude:f32 = input7.trim().parse().expect("Invalid input");

    let aop = base * altitude;
    return aop;
}

fn a_o_c()->f32{
    let mut input8 = String::new();
    println!("\nArea of a cube \nEnter length of the side:");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let l_o_s:f32 = input8.trim().parse().expect("Invalid input");

    let aoc = 6.0 * (l_o_s * l_o_s);
    return aoc;
}

fn v_o_c()->f32{

    let a:f32 = 22.0;
    let b:f32 = 7.0;
    let pi:f32 = a/b;

    let mut input9 = String::new();
    println!("\nVolume of a cylinder \nEnter radius:");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let radius:f32 = input9.trim().parse().expect("Invalid input");

    let mut input10 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input10).expect("Failed to read input");
    let h2:f32 = input10.trim().parse().expect("Invalid input");

    let voc = pi * (radius * radius) * h2;
    return voc;}