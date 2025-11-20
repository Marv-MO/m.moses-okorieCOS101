use std::io;

struct Candidate {
    name: String,
    years_exp: u32,
}

fn main() {
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut input = String::new();
    
    println!("Welcome to EY Nigeria Talent Detector v1.1");
    println!("How many candidates are you registering?");

    io::stdin().read_line(&mut input).expect("I said enter a number!");
    let count: usize = input.trim().parse().expect("Come on, that is not even a number!");
    input.clear();

    for i in 1..=count {
        println!("\nEnter name of candidate {}:", i);
        io::stdin().read_line(&mut input).expect("error");
        let name = input.to_string();
        input.clear();

        println!("How long have you had experience in programming?: ");
        io::stdin().read_line(&mut input).expect("I'm sorry but we can't understand what you entered");
        let years_exp: u32 = input.trim().parse().expect("Your number is incorrect");
        input.clear();

        candidates.push(Candidate { name, years_exp });
    }

    let best_candidate = candidates.iter().max_by_key(|c| c.years_exp).expect("No candidates provided");

    println!("\n  EY NIGERIA ASSESSMENT RESULT  ");
    println!("Most experienced developer: ");
    println!("Name: {}", best_candidate.name);
    println!("Years of programming experience: {}", best_candidate.years_exp);
}
