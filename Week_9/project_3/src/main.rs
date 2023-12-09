use std::io::Write;
use std::io::Read;

fn main() {
    let n_o_c = vec!["\nAigbogun Alamba Daudu","\nMurtala Afeez Bendu","\nOkorocha Calistus Ogbona","\nAdewale Jimoh Akanbi","\nOsazuwa Faith Etieye"];

    let _m = vec!["\nInternal Affairs","\nJustice","\nDefence","\nPower & Steel","\nPetroleum"];

    let g_z = vec!["\nSouth West","\nNorth East","\nSouth South","\nSouth West","\nSouth East"];

    let mut file = std::fs::File::create("EFCC.txt").expect("create failed");

    let i:i32 = 1;
    let j:i32 = 1;
    let k:i32 = 1;
    file.write_all("\nName of Comissioner\t".as_bytes()).expect("write failed");
    for i in n_o_c {
        file.write_all(i.as_bytes()).expect("write failed");
    }
        file.write_all("\nMinistry\t".as_bytes()).expect("write failed");
        for j in &_m{
                    file.write_all(j.as_bytes()).expect("write failed");
        }
            file.write_all("\nGeopolitical Zones\t".as_bytes()).expect("write failed");
             for k in &g_z{
                    file.write_all(k.as_bytes()).expect("write failed");
             }

    println!("\nData written into file.");

}