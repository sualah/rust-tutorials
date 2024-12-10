struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Tyler"),
        sign_in_count: 0,
    };
    println!("{}", user1.username);
    let user2 = build_user(String::from("Tyler2"));
    println!("{}", user2.username);

    let cord = Coordinates(1, 2, 3);
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}
