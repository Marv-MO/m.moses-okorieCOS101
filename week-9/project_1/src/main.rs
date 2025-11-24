use std::io::Write;

fn main() {
    
   let  brewing_company = "Nigerian Brewery Company";

   let mut file = std::fs::File::create("data.txt").expect("failed to create");
   file.write_all("    Lager        |     Stout       |     Non-Alcoholic\n 
   33 Export    |     Legend      |     Maltina\n      
   Desperados   |     Turbo King  |     Amstel Malta\n   
   Goldberg     |     Williams    |     Malta Gold\n    
   Gulder       |                 |     Fayrouz\n 
   Heineken     |                 |            \n  
   Star         |                 |            \n"
       .as_bytes()).expect("write failed");
   file.write_all(brewing_company.as_bytes()).expect("write failed");
   println!("\nData written to file. ");
   
}
