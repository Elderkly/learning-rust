fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 }; //  这里的条件语句需要符合类型一致
    println!("The value of number is: {num}");

    // let num2 = if condition { 5 } else { "six" };   //  将会报错

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 20;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut num3 = 3;
    while num3 != 0 {
        println!("{num3}!");
        num3 -= 1;
    }
    println!("Break while");

    let arr2 = [10, 20, 30, 40, 50];
    for item in arr2 {
        println!("The value is {item}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("Done");
}
