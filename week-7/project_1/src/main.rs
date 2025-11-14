use std::io;
use std::f64::consts::PI;

fn area_of_trapezium(h: f64, a: f64, b: f64) {
    let area = h / 2.0 * (a + b);

    println!("Area of trapezium = {}", area);
}

fn area_of_rhombus(c: f64, d:f64) {
    let area = (c * d) / 2.0;

    println!("Area of rhomus = {}", area);
}

fn area_of_parallelogram(e: f64, f: f64) {
    let area = e * f;

    println!("Area of parallelogram = {}", area);
}

fn area_of_cube(g: f64) {
    let area = 6.0 * g * g;

    println!("Area of cube = {}", area); 
}

fn volume_of_cylinder(j: f64, k: f64) {
    let volume = PI * j * j * k;

    println!("Volume of cylinder = {}", volume);
}

fn get_1_input(label: &str) -> f64 {
    println!("Enter {}:", label);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse().expect("Enter a valid number")
}

fn get_2_inputs(label1: &str, label2: &str) -> (f64, f64) {
    let x1 = get_1_input(label1);
    let x2 = get_1_input(label2);
    (x1,x2) 
}

fn get_3_inputs(label1: &str, label2: &str, label3: &str) -> (f64, f64, f64) {
    let x1 = get_1_input(label1);
    let x2 = get_1_input(label2);
    let x3 = get_1_input(label3);
    (x1,x2,x3)
}

fn main() {
    loop {
        println!("\nShape calculator");
        println!("Select a shape to calculate:");
        println!("1. Area of trapezium");
        println!("2. Area of rhombus");
        println!("3. Area of parallelogram");
        println!("4. Area of cube");
        println!("5. Volume of cylinder");
        println!("0. We are done here");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid input");
        let choice:u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number so we can work");
                continue;
            }
        };
        if choice == 0 {
            println!("See ya!");
            break;
        }

        match choice {
            1 => {
                let (h, a, b) = get_3_inputs("h", "a", "b");
                println!("Area of trapezium");
                area_of_trapezium(h, a, b);
            }
            2 => {
                let (c, d) = get_2_inputs("c", "d");
                println!("Area of rhombus");
                area_of_rhombus(c, d);
            }
            3 => {
                let (e, f) = get_2_inputs("e", "f");
                println!("Area of parallelogram");
                area_of_parallelogram(e, f);
            }
            4 => {
                let g = get_1_input("g");
                println!("Area of cube");
                area_of_cube(g);
            }
            5 => {
                let (j, k) = get_2_inputs("j", "k");
                println!("Volume of cylinder");
                volume_of_cylinder(j, k);
            }
            _ => println!("Wrong choice. Pick the correct one"),
        }
    }    
}
