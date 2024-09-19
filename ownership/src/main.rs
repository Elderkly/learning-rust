fn main() {
    let x = 5;
    print_num(x); //  数字类型实现了Copy特征，不会移动所有权，这里相当于print_num(x.Copy)，所以在这个作用域中x还是有效的
    println!("x is {x}");

    let str = String::from("Hello");
    print_str(str);
    // println!("Double output str:{str}"); //  报错 因为print_str将str的所有权转移进去了

    let str2 = String::from("hello2");
    let (str3, len) = calculate_length(str2);
    println!("The length of '{str3}' is {len}."); //  这里不会报错，因为calculate_length内部将str2的所有权转给了str3
}

fn print_str(str: String) {
    println!("The str is {str}");
} // 这里print_str执行完毕，释放str的内存

fn print_num(num: i32) {
    println!("The num is {num}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
