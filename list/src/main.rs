fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    println!("V1: {:?}", v1);
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    println!("V2: {:?}", v2);

    let v1_item0: &i32 = &v1[0];
    println!("V1 Item 0: {}", v1_item0);

    let v2_item0: Option<&i32> = v2.get(0);
    match v2_item0 {
        Some(value) => println!("V2 Item 0: {}", value),
        None => println!("V2 Item 0: None"),
    }

    let mut v3 = vec![1, 2, 3, 4, 5];
    let v3_item0 = &v3[0];

    // push 操作可能会导致向量重新分配内存（如果当前容量不足），这样可能会使之前的引用 v3_item0 指向无效的内存位置。 所以这里会报错
    // v3.push(6);
    println!("V3 Item 0: {}", v3_item0);

    v3.push(7);
    println!("V3: {:?}", v3);

    for i in &v3 {
        println!("V3 Item: {}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
