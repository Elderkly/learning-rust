use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let y = 6;

    let mut y = y + 1;

    {
        let y = y * 2;
        println!("The value of Y is {y}");
    }

    println!("The value of Y is {y}");

    //  不可改变类型
    // y = "   ";
    // println!("The value of Y is {y}");

    //  元组    可以包含不同类型
    let tup = (500, 6.4, 1);
    let (z1, z2, z3) = tup;
    println!("The value of Z1 is {z1}");

    let tup2 = (10, 20, 30);
    let z11 = tup2.0;
    println!("The value of Z11 is {z11}");

    //  数组    要求相同类型
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5]; //  [3,3,3,3,3]
    let first_a_element = a[0];
    println!("The value of first A element is {first_a_element}");

    let b = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
