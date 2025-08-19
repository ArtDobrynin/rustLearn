fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let max = find_max(&number_list);

    println!("Max number is {}", max);
}

fn find_max(list: &[i32]) -> i32 {
    let mut max= list[0];

    for &number in list.iter() {
        if number > max {
            max = number;
        }
    }

    return max;
}
