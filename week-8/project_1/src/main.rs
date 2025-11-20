use std::io;

struct RoleMapping {
    role : &'static str,
    levels: Vec<&'static str>,    
}

fn main(){

    let roles = vec![RoleMapping {
        role: "Office Administrator",
        levels: vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "DiRector", "CEO"], 
    },
    RoleMapping {
        role: "Academic",
        levels: vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"],
    },
    RoleMapping {
        role: "Lawyer",
        levels: vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"],
    },
    RoleMapping {
        role: "Teacher",
        levels: vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"],
    },
    ];

    let public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES",];

    println!("Enter staff role (e.g. Office Administrator, Lawyer):");
    let mut role_input = String::new();
    io::stdin().read_line(&mut role_input).expect("Please leave!");
    let role = role_input.trim();

    println!("Enter years of experience:");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Unable to read input!");
    let experience: i32 = exp_input.trim().parse().expect("You are not one of us!");

    let aps_index = match experience {
        0..=1 => 0,
        2..=3 => 1,
        4..=5 => 2,
        6..=7 => 3,
        8..=9 => 4,
        _ => 5,
    };

    if let Some(mapping) = roles.iter().find(|r| r.role.eq_ignore_ascii_case(role)) {
        let _staff_position = mapping.levels[aps_index];
        let public_servant = public_servant[aps_index];
        println!("\n  Staff Classification Result  ");
        println!("Role: {}", mapping.role);
        println!("Years of experience: {}", experience);
        println!("Job Position: {}", experience);
        println!("APS Level: {}", public_servant);
    } else {
        println!("Unfortunately, we cannot find you role in this place. Bye!");
    }

}
