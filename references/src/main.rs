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

// 快速排序
fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = len - 1;
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}
