use std::io::Write;

fn main() {
    
    let mut file = std::fs::File::create("Ministers from different geopolitical zones").expect("creation failed");
    file.write_all("S/N  |  Name of Commisioner  |  Ministry  |  Geopolitical zone\n"
        .as_bytes())
        .expect("failed to write");

    let s_n = vec!["1", "2", "3", "4", "5"];
    let comm_name = vec!["Aigbogun Alamba Dauda", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye",];
    let min = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geo_zone = vec!["South West", "North East", "South South", "South West", "South East"];


    for i in 0.. {
        let line = format!("{}  |  {}  |  {}  |  {}\n", s_n[i], comm_name[i], min[i], geo_zone[i]);
        file.write_all(line.as_bytes()).expect("failed to write");
    }
    println!("\nData written to file sucessfully");
}
