fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut max = number_list[0];

    for number in number_list {
        if number > max {
            max = number;
        }
    }
}
