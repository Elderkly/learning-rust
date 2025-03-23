/**
 * 生命周期
 * 1.所有权
 * 2.生命周期注解
 * 3.生命周期范围
 * 4.编译器关于生命周期的三条规则
 * 5.静态生命周期'static
*/

fn main() {
    let s1 = String::from("str1");
    let result;
    {
        let s2 = String::from("str222");
        result = longest_str(s1.as_str(), s2.as_str());
        println!("The longest string is {}", result);
    }
    println!("The longest string is {}", result);
}

//  声明周期注解  &'{name} {type}
//  这里返回的引用只会保留x和y中生命周期较短的一个
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
