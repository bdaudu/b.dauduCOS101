use std::io::Write;
use std::io::Read;

fn main() {
    let s_n = vec!["Oluchi Mordi","Adams Aliyu","Shania Boalde","Adekunle Gold","Blanca Edemoh"];

    let m_n = vec!["ACC102111","ECO10110101","CSC10328828","EE11020202","MEE10202001"];

    let d = vec!["Accounting","Economics","Computer Science","Electrical Engineering","Mechanical Engineering"];

    let level = vec!["300","100","200","200","100"];

    let mut file = std::fs::File::create("pausims.txt").expect("create failed");

    file.write_all("\tSTUDENT NAME\t".as_bytes()).expect("write failed");
    file.write_all("\tMATRIC. NUMBER\t".as_bytes()).expect("write failed");
    file.write_all("\tDEPARTMENT\t".as_bytes()).expect("write failed");
    file.write_all("\tLEVEL\t".as_bytes()).expect("write failed");
    for i in s_n {
         file.write_all(i.as_bytes()).expect("write failed");
    }

    for j in m_n{
        file.write_all(j.as_bytes()).expect("write failed");
    }

    for k in d{
        file.write_all(k.as_bytes()).expect("write failed");
    }

    for g in level{
        file.write_all(g.as_bytes()).expect("write failed");
    }
    println!("\nData written into file.");

}
