fn main() {
    let s = String::from("lipsy");
    print_programm(s);

    let x = true;
    check_variable(x);

    println!("{}", x);
}

fn print_programm(user_string: String) {
    println!("{}", user_string);
}

fn check_variable(check: bool) {
    println!("{}", check);
}