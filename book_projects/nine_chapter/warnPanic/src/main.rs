use std::fs::File;

fn main() {
    let f = File::open("file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Нет файла {}", error)
        },
    };
}
