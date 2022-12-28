// 借用

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let res = largest(&number_list);
    println!("{}", res);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let res = largest(&char_list);
    println!("{}", res);
}

// fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

/*
    如果并不希望限制 largest 函数只能用于实现了 Copy 特征的类型，
    我们可以在 T 的特征约束中指定 Clone 特征 而不是 Copy 特征。
    并克隆 list 中的每一个值使得 largest 函数拥有其所有权。
    使用 clone 函数意味着对于类似 String 这样拥有堆上数据的类型，
    会潜在地分配更多堆上空间，而堆分配在涉及大量数据时可能会相当缓慢。

    另一种 largest 的实现方式是返回在 list 中 T 值的引用。如果我们将函数返回值从 T 改为 &T
    并改变函数体使其能够返回一个引用，我们将不需要任何 Clone 或 Copy 的特征约束而且也不会有任何的堆分配。
    尝试自己实现这种替代解决方式吧！

*/
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
