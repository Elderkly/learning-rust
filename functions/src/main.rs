fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 3;
        x + 1 //  这里没有; 表示是一个表达式，会有返回值，如果带有；则为语句，不会有返回值
    };
    println!("The value of y is {y}");

    let function_return_value = five();

    println!("The function return value is {function_return_value}");

    let plus_fn_return_value = plus(5, 6);

    println!("The function plus return value is {plus_fn_return_value}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn five() -> i32 {
    5
}

fn plus(x: i32, y: i32) -> i32 {
    // x + y;  //   这么写将会报错
    x + y
}
