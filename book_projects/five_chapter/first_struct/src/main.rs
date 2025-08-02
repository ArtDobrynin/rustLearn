struct User<'a> {
    id: u32,
    name: &'a str,
    email: String,
    hash_password: String
}

fn main() {
    let user_toha = User {
        id: 666,
        email: String::from("bashmachok@gmail.com"),
        hash_password: String::from("@notfoundpassword123"),
        name: "Toha"
    }

    println!("Name: {}", user_toha.name);
    println!("Id: {}", user_toha.id);
}
