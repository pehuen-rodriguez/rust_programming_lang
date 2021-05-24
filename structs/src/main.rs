fn main() {
    let user1 = User {
        username: String::from("juanca"),
        email: String::from("juanca@gmail.com"),
        active: true,
        sign_in_count: 1
    };

    let mut user2 = User {
        username: String::from("marleen"),
        email: String::from("marllen@gmail.com"),
        active: true,
        sign_in_count: 10
    };

    user2.username = user1.username;
    user2.email = user1.email;
    user2.active = user1.active;
    user2.sign_in_count = user1.sign_in_count;
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
