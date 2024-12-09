use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    //  主动抛出错误
    // panic!("crash and burn");

    // 越界访问
    // let v = vec![1, 2, 3];
    // v[99];

    // 捕获错误
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {}", e),
    //         },
    //         other_error => panic!("Error opening file: {}", other_error),
    //     },
    // };
    // println!("greeting_file: {:?}", greeting_file);

    // 如果找不到这个文件就报错 相当于做了match 但是报错信息不能自定义
    // let greeting_file = File::open("hello2.txt").unwrap();

    // 相当于catch 可以自定义报错信息
    // let greeting_file =
    //     File::open("hello2.txt").expect("hello2.txt should be included in this project");

    let username_file_result = File::open("username.txt");

    // 这里定义为mut是因为后面的read_to_string读取文件时 会修改文件内部的指针 所以需要是mut
    let mut username_file_info = match username_file_result {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    println!("username_file_info: {:?}", username_file_info);

    let mut username1 = String::new();
    username_file_info
        .read_to_string(&mut username1)
        .expect("Failed to read username");

    println!("username1: {}", username1);

    //  上面读取username的简化版
    let username2 = read_username_from_file().unwrap();
    println!("username2: {}", username2);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
