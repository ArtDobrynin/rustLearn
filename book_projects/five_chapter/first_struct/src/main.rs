struct User {
    id: u32,
    email: String,
    hash_password: String
}

struct Player {
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

    let player_toha = Player {
        id: 777,
        ..user_toha
    };

    println!("hash_password: {}", player_toha.hash_password);
    println!("Id: {}", user_toha.id);
}
