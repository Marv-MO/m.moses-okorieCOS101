use std::f64::consts::PI;

fn area_of_trapezium(h: i32, a: i32, b: i32) {
    let area = h / 2 * (a + b);

    println!("Area of trapezium = {}", area);
}

fn area_of_rhombus(c: i32, d:i32) {
    let area = (c * d) / 2;

    println!("Area of rhomus = {}", area);
}

fn area_of_parallelogram(e: i32, f: i32) {
    let area = e * f;

    println!("Area of parallelogram = {}", area);
}

fn area_of_cube(g: i32) {
    let area = 6 * g * g;

    println!("Area of cube = {}", area); 
}

fn volume_of_cylinder(j: f64, k: f64) {
    let volume = PI * j * j * k;

    println!("Volume of cylinder = {}", volume);
}

fn main() {
    let mut input1 = String::new();
    println!("Enter input for parameter H:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let h:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter A:");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let a:i32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter input for parameter B:");
    std::io::stdin().read_line(&mut input3).expect("Failed to read input");
    let b:i32 = input3.trim().parse().expect("Invalid input");

    // call add function with argument
    area_of_trapezium(h, a, b);

    let mut input4 = String::new();
    println!("Enter input parameter C:");
    std::io::stdin().read_line(&mut input4).expect("Failed to read input");
    let c:i32 = input4.trim().parse().expect("Invalid input");

    let mut input5 = String::new();
    println!("Enter input for parameter D:");
    std::io::stdin().read_line(&mut input5).expect("Failed to read input");
    let d:i32 = input5.trim().parse().expect("Invalid input");

    // call add function with argument
    area_of_rhombus(c, d);

    let mut input6 = String::new();
    println!("Enter input for parameter E:");
    std::io::stdin().read_line(&mut input6).expect("Failed to read input");
    let e:i32 = input6.trim().parse().expect("Invalid input");

    let mut input7 = String::new();
    println!("Enter input for parameter F:");
    std::io::stdin().read_line(&mut input7).expect("Failed to read input");
    let f:i32 = input7.trim().parse().expect("Invalid input");

    // call add function with argument
    area_of_parallelogram(e, f);

    let mut input8 = String::new();
    println!("Enter input for parameter G:");
    std::io::stdin().read_line(&mut input8).expect("Failed to read input");
    let g:i32 = input8.trim().parse().expect("Invalid input");

    // call add function with argument
    area_of_cube(g);

    let mut input9 = String::new();
    println!("Enter input for parameter J:");
    std::io::stdin().read_line(&mut input9).expect("Failed to read input");
    let j:f64 = input9.trim().parse().expect("Invalid input");

    let mut input10 = String::new();
    println!("Enter input for parameter K:");
    std::io::stdin().read_line(&mut input10).expect("Failed to read input");
    let k:f64 = input10.trim().parse().expect("Invalid input");

    // call add function with argument
    volume_of_cylinder(j, k);
}