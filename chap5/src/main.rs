#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn create_square(dim: u32) -> Self {
        Self {
            width: dim,
            height: dim,
        }
    }
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 60,
    };
    println!("The Area of rectangle is {}", rect.area());
    let sqaure = Rectangle::create_square(6);
    println!("{:?}", rect);
    println!("{:?}", sqaure);
    // let user = build_user("some".to_string(), "someName".to_string());
    // println!("{:#?}", user);
    // dbg!(&user);
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };
    // user1.active = false;
    // println!("{}", user1.active);
    // let user2 = build_user("some@gmail.com".to_string(), "responsetime".to_string());
    // println!("{}", user2.email);
    // let user3 = User {
    //     username: String::from("new user"),
    //     ..user1
    // };
    // println!("{}", user3.username);
}
