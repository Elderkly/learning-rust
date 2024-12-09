fn main() {
    // 字符串字面量
    let mut str = "Hello World";
    // String 类型
    let s1 = str.to_string();
    let mut s2 = String::from("Hello World");
    println!("{}", s1);

    s2.push_str("!");
    println!("{}", s2);

    let s3 = s1 + &s2;
    println!("{}", s3);

    // println!("{}", s1); // 报错 因为s1的所有权转给s3了

    let s4 = format!("{s2}-{s3}");
    println!("{}", s4);

    let s5 = "中国人";
    // let s6 = &s5[0..2];     //  报错  因为一个中文字符占三个字节
    let s6 = &s5[0..3];
    println!("{}", s6);

    for c in s5.chars() {
        println!("{}", c);
    }
}
