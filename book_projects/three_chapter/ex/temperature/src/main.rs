use std::io;

fn main() {
    let fahr_temp = loop{
        let mut celsi_temp = String::new(); 
        io::stdin().read_line(&mut celsi_temp)
            .expect("Not string!");
        
        let celsi_temp: f64 = match celsi_temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Get have number!");
                continue;
            } 
        };

        break get_fahr(celsi_temp);
    };

    println!("Equal Fahr temp: {}", fahr_temp);
}

fn get_fahr(celsi_temp: f64) -> f64 {
    const VALUE_TEMP: f64 = 9.0 / 5.0;
    VALUE_TEMP * celsi_temp + 32.0
}