fn main() {
    let v: Vec<i32> = Vec::new();
    println!("Hello, world! {}", v.len());
    let mut v1 = Vec::with_capacity(10);
    v1.push(1);
    v1.push(2);
    v1.push(1);
    v1.push(2);
    println!("Hello, world! {}", v1.len());
    let v2 = vec![1, 2, 3, 4, 5];

    let third = &v2[2];
    println!("The third element is {}", third);
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];

    let first = &v[0];

    v.push(6);
    /*

        其实，按理来说，这两个引用不应该互相影响的：一个是查询元素，一个是在数组尾部插入元素，完全不相干的操作，为何编译器要这么严格呢？

    原因在于：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存，这非常 rusty —— 对用户进行严格的教育。

    其实想想，在长大之后，我们感激人生路上遇到过的严师益友，正是因为他们，我们才在正确的道路上不断前行，虽然在那个时候，并不能理解他们，而 Rust 就如那个良师益友，它不断的在纠正我们不好的编程习惯，直到某一天，你发现自己能写出一次性通过的漂亮代码时，就能明白它的良苦用

        */
    // println!("The first element is: {}", first);
}
