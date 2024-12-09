use std::collections::HashMap;
fn main() {
    test1();
}

fn test1() {
    let mut v = Vec::from([10, 5, 8, 9, 11, 6, 0, 11]);
    fast_sort(&mut v);
    let middle_index = v.len() / 2;
    println!("v: {:?}", v);
    println!("中位数：{}", v[middle_index]);

    let mut max_key = v[0];
    let mut max_value = 0;
    for &i in &v {
        println!("i: {}", i);
        println!("max_key: {}", max_key);
        let count = if i == max_key { max_value + 1 } else { 1 };
        println!("count: {}", count);
        if count >= max_value {
            max_value = count;
            max_key = i;
        }
    }

    println!("众数: {}", max_key);
}
fn fast_sort(v: &mut [i32]) {
    if v.len() <= 1 {
        return;
    }

    let pivot = partition(v);

    // 只有当子数组长度大于0时才进行递归
    if pivot > 0 {
        fast_sort(&mut v[0..pivot]);
    }
    if pivot + 1 < v.len() {
        fast_sort(&mut v[pivot + 1..]);
    }
}

fn partition(v: &mut [i32]) -> usize {
    let last = v.len() - 1; // 使用一个变量存储最后的索引，提高代码可读性
    let mut i = 0;

    for j in 0..last {
        // 改用 last 而不是 pivot 作为循环边界
        if v[j] < v[last] {
            v.swap(i, j);
            i += 1;
        }
    }

    v.swap(i, last); // 使用 last 替换 pivot
    i // 去掉显式的 return，使用 Rust 的隐式返回
}
