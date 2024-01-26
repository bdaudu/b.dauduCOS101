use std::io;
use std::fs::File;
use std::io::Write;

struct Company{
    shares:u32,
    date:u32,
    liabilities:u32,
    username:String,
    password:String
}

impl Company{
    fn plc(&self)->u32{
        (self.shares - self.liabilities/self.shares) * 100
    }
    fn f_p(&self)->u32{
        self.plc() * 0.05 as u32
    }

}
fn main() {
    let cadbury = Company{
        shares:15_000_000,
        date:1965,
        liabilities:5_500_000,
        username:String::from("cadb"),
        password:String::from("cadry965"),
        
    };
    let champ = Company{
        shares:25_000_000,
        date:1974,
        liabilities:8_000_000,
        username:String::from("cham"),
        password:String::from("chaon974"),
        
    };
    let dangote = Company{
        shares:18_000_000,
        date:1970,
        liabilities:10_000_000,
        username:String::from("dang"),
        password:String::from("dante970")
    };
    let flourmills = Company{
        shares:32_000_000,
        date:1960,
        liabilities:4_000_000,
        username:String::from("flou"),
        password:String::from("flols960"),
    };
    let nestle = Company{
        shares:8_000_000,
        date:1961,
        liabilities:1_500_000,
        username:String::from("nest"),
        password:String::from("nesle961"),
        
    };
    let unilever = Company{
        shares:37_000_000,
        date:1923,
        liabilities:11_000_000,
        username:String::from("unil"),
        password:String::from("unier923")
    };
    let honeywell = Company{
        shares:34_000_000,
        date:1906,
        liabilities:9_000_000,
        username:String::from("hone"),
        password:String::from("honll906")
    };
    let nigeria_breweries = Company{
        shares:30_000_000,
        date:1946,
        liabilities:12_000_000,
        username:String::from("nige"),
        password:String::from("niges946")
    };

    println!("Enter in the company's username:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Not a valid string");
    let username:String = input1;

    println!("Enter in the password:");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Not a valid string");
    let password:String = input2;

    if username == "cadb" && password == "cadry965" && password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else if username == "cham" && password == "chaon974"&& password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else if username == "dang" && password == "dante970"&& password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else if username == "flou" && password == "flols960"&& password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else if username == "nest" && password == "nesle961"&& password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else if username == "unil" && password == "unier923"&& password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else if username == "hone" && password == "honll906"&& password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else if username == "nige" && password == "niges946"&& password.len() >=3 && password.len() <=8{
        println!("Access granted!");
    }
    else{
        println!("Access denied!\nRequirements for password:\nLetters should be between a-z\nNumbers should be between 0-9\nNo special characters($@#)\nMinimum length:3\nMaximum length:8");
    }

    let mut file = std::fs::File::create("COMPANIES.txt").expect("create failed");
    file.write_all(b"COMPANIES\t\t\tFOUNDING DATE\tSHARES(ASSETS)\tLIABILITIES\tPERCENTAGE LEVERAGES(%)
                    \nCadbury Nigeria Plc\t\t\t1965\t15,000,000\t5,500,000\t63.30
                    \nChampion Breweries Plc\t\t\t1974\t25,000,000\t8,000,000\t68.00
                    \nDangote Sugar Refinery Plc\t\t1970\t18,000,000\t10,000,000\t44.40
                    \nFlour Mills Nigeria Plc\t\t\t1960\t32,000,000\t4,000,000\t87.50
                    \nNestle Nigeria Plc\t\t\t1961\t8,000,000\t1,500,000\t81.25
                    \nUnilever Nigeria Plc\t\t\t1923\t37,000,000\t11,000,000\t70.27
                    \nHoneywell Nigeria Plc\t\t\t1906\t34,000,000\t9,000,000\t73.53
                    \nNigerian Breweries Plc\t\t\t1946\t30,000,000\t12,000,000\t60.00");
    println!("Data written into file ");

    let mut second_file = std::fs::File::create("Companies with shares higher than 20,000,000.txt").expect("create failed");
    second_file.write_all(b"COMPANIES\t\t\tPERCENTAGE LEVERAGES
                            \nChampion Breweries Plc\t\t\t68.00
                            \nFlour Mills Nigeria Plc\t\t\t87.50
                            \nUnilever Nigeria Plc\t\t\t70.27
                            \nHoneywell Nigeria Plc\t\t\t73.53
                            \nNigerian Breweries Plc\t\t\t60.00");
    
}
