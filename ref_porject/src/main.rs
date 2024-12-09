fn main() {
    let x = 5;
    let y = &x;

    println!("x: {}", x);
    println!("y: {}", y); //  自动解引用 输出5
    println!("&y: {:p}", y); //  打印内存地址

    let mut z = 6;
    let o = &mut z;

    // o = 5; //  不能直接修改o指向的地址

    *o = 1; // 解引用后修改到z  这里z需要是mut类型

    println!("z: {}", z);

    let mut x1 = 5;
    let mut x2 = &mut x1;
    let x3 = &mut x2;

    **x3 = 10; //  多重引用

    println!("x1: {}", x1);
}
