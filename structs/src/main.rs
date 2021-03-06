fn main() {
    let user1 = User {
        username: String::from("juanca"),
        email: String::from("juanca@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        username: String::from("marleen"),
        email: String::from("marllen@gmail.com"),
        active: true,
        sign_in_count: 10,
    };

    user2.username = user1.username;
    user2.email = user1.email;
    user2.active = user1.active;
    user2.sign_in_count = user1.sign_in_count;

    let user3 = build_struct(String::from("coroto"), String::from("coroto@gmail.com"));
    println!("User3 build with username: {}", user3.username);

    let user4 = User {
        username: String::from("adrian"),
        email: String::from("adrian@gmail.com"),
        ..user3
    };
    println!("User4 build with struct update sintax: {}", user4.username);
    let center = Point(0, 0, 0);
    println!("Built center point with x: {}", center.0);
}

fn build_struct(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

struct Point(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
