use std::io;

fn main() 
{
    let mut input = String::new();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let age:f32 = input.trim().parse().expect("Not a valid number");

    if age >= 40.0 
    {
        println!("The incentive is 1560000.0");
    }
    else if age > 30.0 && age < 40.0 
    {
        println!("The incentive should be 1480000.0");
    }
    else if age < 28.0
    {
        println!("The incentive should be 1300000.0 per month");
    }
    else 
    {
        println!("The incentive should be 100000.0");
    }
}
