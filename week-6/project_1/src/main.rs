use std::io;

fn main() {
    println!("\n Welcome to Moses Food joint!");

    
    println!("");
    println!("             FOOD MENU              ");
    println!("");
    println!("P = Poundo Yam/Edinkaiko Soup - 3200");
    println!("F = Fried Rice & Chicken - 3000");
    println!("A = Amala & Ewedu Soup - 2500");
    println!("E = Eba & Egusi Soup - 2000");
    println!("W = White Rice & Stew - 2500");
    println!("Q = quit");
    let mut total_cost:f64 = 0.0;

    loop {
        println!("Enter food type (P, F, A, E, W, Q): ");
        let mut food_type = String::new();
        io::stdin().read_line(&mut food_type).expect("Failed to read input");
        let food_type = food_type.trim().to_uppercase();

        if food_type == "Q" {
            break;
        }

        println!("Enter quantity: ");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Invalid input");
        let qty:u32 = quantity.trim().parse().expect("Invalid number");
        let (food_name, price): (&str, f64) = match food_type.as_str() {
            "P" => ("Poundo Yam/Edinkaiko Soup", 3200.0),
            "F" => ("Fried Rice & Chicken", 3000.0),
            "A" => ("Amala & Ewedu Soup", 2500.0),
            "E" => ("Eba & Egusi Soup", 2000.0),
            "W" => ("White Rice & Stew", 2500.0),
            _ => {
                println!("Please select a valid food type");
                continue;
            }
    };

    let total = price * qty as f64;
    total_cost += total;

    let final_amount = if total_cost > 10000.0 {
        let discount = 0.05 * total;
        println!("Discount applied: {:.2}",discount);
        total_cost - discount
    } else {
        total
    };

    println!("");
    println!("              Order Summary             ");
    println!("");
    println!("Food: {}", food_name);
    println!("Quantity: {}", quantity);
    println!("Total (before discount): {:.2}", total);
    println!("Total cost (before discount): {:.2}", total_cost);

    if total > 10000.0 {
        println!("Discount (5%): {:.2}", total * 0.05);
    }
    println!("Final Amount to Pay: {:.2}", final_amount);}
    println!("======================================");
    println!("God bless you for ordering at the best food joint ever");
}   
