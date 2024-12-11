// struct User {
//     active: bool,
//     username: String,
//     sign_in_count: u32,
// }

// struct Coordinates(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(15);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("The area of rect1 is {} square pixels", rect1.area());
    println!("The area of square1 is {} square pixels", square1.area());

    // let user1 = User {
    //     active: true,
    //     username: String::from("Tyler"),
    //     sign_in_count: 0,
    // };
    // println!("{}", user1.username);
    // let user2 = build_user(String::from("Tyler2"));
    // println!("{}", user2.username);

    // let cord = Coordinates(1, 2, 3);
}

// fn build_user(username: String) -> User {
//     User {
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
