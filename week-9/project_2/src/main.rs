use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("creation failed");
    file.write_all("  Student Name  |  Matric. Number  |  Department  |  Level\n"
        .as_bytes())
        .expect("write failed");

    let stud_name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_num = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"];


    for i in 0..5 {
        let line = format!("  {}  |  {}  |  {}  |  {}\n", stud_name[i], matric_num[i], dept[i], level[i]);
        file.write_all(line.as_bytes()).expect("write failed");
    }
    println!("\nData written to file sucessfully"); 
}
