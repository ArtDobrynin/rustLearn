use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    let message = read_username_from_file();

    println!("Message: {:?}", message);
}

fn read_username_from_file () -> Result<String, io::Error> {
    let f = File::open("file.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
