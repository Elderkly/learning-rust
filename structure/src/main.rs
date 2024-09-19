struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

#[derive(Debug)] //  方便调试时打印结构体
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = build_user(String::from("888@qq.com"), String::from("my name"));
    println!("user1.username: {:?}", user1.username);

    let user2 = User {
        username: String::from("name2"),
        ..user1
    };

    println!("user2.username: {}", user2.username);

    //  这里会报错，因为user1.email的所有权转移给了user2，所以不能再访问user1.email
    // println!("user1.email:{}; user2.email:{}", user1.email, user2.email);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        dbg!(get_area(&rect1))
    );

    println!("rect1: {:#?}", rect1);

    dbg!(&rect1);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn get_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
