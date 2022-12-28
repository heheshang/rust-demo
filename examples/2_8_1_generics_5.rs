//泛型

fn main() {
    let arr = [1, 2, 3];
    display_arr(&arr);
    display_arr1(&arr);
    let arr = [1, 2];
    display_arr(&arr);
    display_arr1(&arr);
    let arr = [1, 2, 3, 4];
    display_arr(&arr);
    let char_arr = ['a', 'b', 'c'];
    display_arr1(&char_arr);

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

// fn display_arr(arr: [i32; 3]) {
//     println!("{:?}", arr);
// }
/*
    结合代码和报错，可以很清楚的看出，[i32; 3] 和 [i32; 2] 确实是两个完全不同的类型，因此无法用同一个函数调用。

    首先，让我们修改代码，让 display_array 能打印任意长度的 i32 数组：

*/
fn display_arr(arr: &[i32]) {
    println!("{:?}", arr);
}

fn display_arr1<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
