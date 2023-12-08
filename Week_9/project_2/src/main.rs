use std::io::Write;
use std::io::Read;

fn main() {
    let s_n = vec!["\nOluchi Mordi","\nAdams Aliyu","\nShania Boalde","\nAdekunle Gold","\nBlanca Edemoh"];

    let m_n = vec!["\nACC1021111","\nECO10110101","\nCSC10328828","\nEE11020202","\nMEE10202001"];

    let d = vec!["\nAccounting","\nEconomics","\nComputer Science","\nElectrical Engineering","\nMechanical Engineering"];

    let level = vec!["\n300","\n100","\n200","\n200","\n100"];

    let mut file = std::fs::File::create("pausims.txt").expect("create failed");

    file.write_all("\nSTUDENT NAME\t".as_bytes()).expect("write failed");
    for i in s_n {
         file.write_all(i.as_bytes()).expect("write failed");
    }

    file.write_all("\nMATRIC. NUMBER\t".as_bytes()).expect("write failed");
    for j in m_n{
        file.write_all(j.as_bytes()).expect("write failed");
    }

    file.write_all("\nDEPARTMENT\t".as_bytes()).expect("write failed");
    for k in d{
        file.write_all(k.as_bytes()).expect("write failed");
    }

    file.write_all("\nLEVEL\t".as_bytes()).expect("write failed");
    for g in level{
        file.write_all(g.as_bytes()).expect("write failed");
    }
    println!("\nData written into file.");

}
