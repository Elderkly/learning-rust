enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 0,
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(6);
    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; //  这里会报错 因为i8类型和Option<i8>类型不能直接运算

    let mut count1 = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    let count2 = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!")
    } else {
        count2 += 1;
    }

    //  上面两段代码等价
}
