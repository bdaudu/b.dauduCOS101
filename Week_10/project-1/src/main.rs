struct Laptop{
    amount:u32,
    price:u32

}

impl Laptop{
    fn total(&self)->f32{
        (self.amount * self.price) as f32
    }
}

fn main() {
    let hp = Laptop{
        amount:3,
        price:650_000
    };

    let ibm = Laptop{
        amount:3,
        price:755_000
    };

    let tosh = Laptop{
        amount:3,
        price:550_000
    };

    let dell = Laptop{
        amount:3,
        price:850_000
    };

    println!("Cost for all laptops:\nHP = N{}\nIBM = N{}\nToshiba = N{}\nDell = N{}\nTotal = N{}",hp.total(),ibm.total(),tosh.total(),dell.total(),hp.total()+ibm.total()+tosh.total()+dell.total());
}
