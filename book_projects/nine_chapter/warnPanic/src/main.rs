use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(file) => file,
                Err(e) => panic!("Error message: {:?}", e),
            },
            other_error => panic!("Error message: {:?}", other_error),
        },
    };
}
