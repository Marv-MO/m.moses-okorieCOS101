struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn fn_cost_for(&self, quantity: u32) -> u32 {
        self.price * quantity

    }
}

fn main() {
    let hp = Laptop {
        brand:String::from("HP"),
        price:650000
    };
    let ibm = Laptop {
        brand:String::from("IBM"),
        price:755000
    };
    let toshiba = Laptop {
        brand:String::from("TOSHBA"),
        price:550000
    };
    let dell = Laptop {
        brand:String::from("DELL"),
        price:850000
    };

    let qty = 3;

    let total = hp.fn_cost_for(qty) + ibm.fn_cost_for(qty) + toshiba.fn_cost_for(qty) + dell.fn_cost_for(qty);

    println!("Total cost for 3 units of each brand = ₦{}", total);
}