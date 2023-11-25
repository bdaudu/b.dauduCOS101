use std::io;

fn main() {
   let mut input1 = String::new();
   println!("Please input your choice of formula: \nArea of Trapezium = 1 \nArea of Rhombus = 2 \nArea of Parallelogram = 3 \nArea of Cube = 4 \nVolume of Cylinder = 5");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let choice:i32 = input1.trim().parse().expect("Invalid input");

   if choice == 1{
    println!("{} squared",a_o_f());
   }
   else if choice == 2{
    println!("{} squared",r_f())
   }
   else if choice == 3{
    println!("{} squared",a_o_p())
   }
   else if choice == 4{
    println!("{} squared",a_o_c())
   }
   else if choice == 5{
    println!("{} cubed",v_o_c())
   }
   else{
    println!("Please input a number from 1 to 5");
   }
}

fn a_o_f()->f32{
    let mut input2 = String::new();
    println!("Area of a trapezium \nEnter base 1:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter base 2:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2:f32 = input3.trim().parse().expect("Invalid input");

    let mut input4 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let height:f32 = input4.trim().parse().expect("Invalid input");

    let aof = height/(2.0 * (base1 + base2));
    println!("Area of Trazepium: {}",aof);
    return aof;
}

fn r_f()->f32{
    let mut input5 = String::new();
    println!("\nArea of a rhombus \nEnter diagonal 1:");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let diagonal1:f32 = input5.trim().parse().expect("Invalid input");

    let mut input6 = String::new();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let diagonal2:f32 = input6.trim().parse().expect("Invalid input");

    let rf = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus: {}",rf);
    return rf;
}

fn a_o_p()->f32{
    let mut input7 = String::new();
    println!("\nArea of a parallelogram \nEnter base:");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let base:f32 = input7.trim().parse().expect("Invalid input");

    let mut input8 = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let altitude:f32 = input8.trim().parse().expect("Invalid input");

    let aop = base * altitude;
    println!("Area of Parallelogram: {}",aop);
    return aop;
}

fn a_o_c()->f32{
    let mut input9 = String::new();
    println!("\nArea of a cube \nEnter length of the side:");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let l_o_s:f32 = input9.trim().parse().expect("Invalid input");

    let aoc = 6.0 * (l_o_s * l_o_s);
    println!("Area of Cube: {}",aoc);
    return aoc;
}

fn v_o_c()->f32{

    let a:f32 = 22.0;
    let b:f32 = 7.0;
    let pi:f32 = a/b;

    let mut input10 = String::new();
    println!("\nVolume of a cylinder \nEnter radius:");
    io::stdin().read_line(&mut input10).expect("Failed to read input");
    let radius:f32 = input10.trim().parse().expect("Invalid input");

    let mut input11 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input11).expect("Failed to read input");
    let h2:f32 = input11.trim().parse().expect("Invalid input");

    let voc = pi * (radius * radius) * h2;
    println!("Volume of Cylinder: {}",voc);
    return voc;
}