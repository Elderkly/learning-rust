use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);

    // println!("{}", field_name); // 报错 因为所有权已经转移

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);
    //  Blue 的值会被覆盖
    println!("{:?}", scores2);

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    // 如果键不存在，则插入值
    scores3.entry(String::from("Yellow")).or_insert(50);
    // 如果键存在，则不插入
    scores3.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores3);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // split_whitespace 会按空格分割字符串
    for word in text.split_whitespace() {
        // or_insert 返回一个可变引用
        let count = map.entry(word).or_insert(0);
        // 解引用并加1
        *count += 1;
    }

    println!("{:?}", map);
}
