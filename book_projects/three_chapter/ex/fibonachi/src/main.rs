fn main() {
    let fibo = get_fibo(7);
    println!("Number fibo: {}", fibo);
}

fn get_fibo(mut number: i32) -> i32 {
    let mut number_one = 0;
    let mut number_two = 1;
    while number != 2 {
        let number_part = number_two;

        number_two = number_one + number_two;
        number_one = number_part;

        number -= 1;
    }

    return number_one + number_two;
}
