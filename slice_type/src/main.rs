fn main() {
    let mut s = String::from("Hello world");

    let hello1 = &s[0..5];
    let hello2 = &s[..5];

    let world1 = &s[6..s.len()];
    let world2 = &s[6..];

    println!("hello1: {hello1} hello2: {hello2}; world1: {world1} world2: {world2}");

    let full_text1 = &s[0..s.len()];
    let full_text2 = &s[..];

    // s.clear(); //  这里会报错，因为full_text保存的是s的引用，这里把s清空了会导致full_text也被清空

    println!("fullText1: {full_text1} fullText2: {full_text2}");

    //  数组也支持切片
    let a = [1, 2, 3, 4, 5];
    //  取索引1~3(不包含3)的数据
    let slice = &a[1..3];

    println!("slice: {:?}", slice);
}
