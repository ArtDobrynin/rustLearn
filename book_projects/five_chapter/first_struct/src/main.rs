#[derive (Debug)]
struct User {
    id: u32,
    email: String,
    hash_password: String
}

fn main() {
    let user_toha = User {
        id: 666,
        email: String::from("bashmachok@gmail.com"),
        hash_password: String::from("@notfoundpassword123"),
    };

    println!("Id: {:#?}", user_toha);
}
