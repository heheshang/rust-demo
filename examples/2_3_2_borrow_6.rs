// 借用
/*
    借用规则总结
    总的来说，借用规则如下：

    同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    引用必须总是有效的
*/
fn main() {
    // let references_to_nothing = dangle();
    let references_to_nothing = no_dangle();
    println!("{}", references_to_nothing);
}
// fn dangle() -> &'static String {
//     let s = String::from("hello");
//     &s
// }

/*
    因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放, 但是此时我们又尝试去返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！

    其中一个很好的解决方法是直接返回 String：

*/
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
