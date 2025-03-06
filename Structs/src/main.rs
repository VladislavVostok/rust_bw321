
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

#[derive(Debug, Clone)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {

    fn new(username:String, email:String)->Self{
        User{
            active: true,
            username,
            email,
            sign_in_count: 123
        }
    }

}
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Hello, world!");



    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");


    let user2 = User::new(String::from("user"), String::from("test"));
    let str = String::new();

    let rect = Rectangle{
        width: 30,
        height: 30
    };

    if rect.width() {
        println!("{:#?}", rect);
    }

    println!("{:?}", user1);

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{four:?}")

}
