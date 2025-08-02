fn main() {
    let full_strings = String::from("Alesha Popovich");
    let first_string = first_word(&full_strings);

    println!("{}", first_string);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}