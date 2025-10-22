use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let y:f32 = (b * b) - (4.0 * a * c);

    if y > 0.0 {
        let root_1 = (-b + y.sqrt()) / (2.0 * a);
        let root_2 = (-b - y.sqrt()) / (2.0 * a);
        println!("These are the two distinct roots; {} and {}", root_1, root_2);
    } 
    else if y == 0.0 {
        let root = (-b) / (2.0 * a);
        println!("There only root is; {}", root);
    }
    else {
        println!("No real root");
    }
}