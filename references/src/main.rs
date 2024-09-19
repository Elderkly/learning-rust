fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //  这里传递的是s1的引用，所以在这个作用域s1的所有权还是有效的
    println!("The length of '{s1}' is {len}.");

    let s2 = String::from("hello");
    // change(&s2); //  这里会报错，因为传入的引用默认是不允许修改的

    let mut s3 = String::from("hello");
    change2(&mut s3); //  这里传入的是mut的引用 允许修改

    let r3 = &mut s3;
    let r4 = &mut s3;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}
