// 借用

fn main() {
    let s1 = String::from("hello   ");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
/*

    无需像上章一样：先通过函数参数传入所有权，然后再通过函数返回来传出所有权，代码更加简洁
    calculate_length 的参数 s 类型从 String 变为 &String
*/
// s 是对 String 的引用
fn calculate_length(s: &String) -> usize {
    s.len()
}
// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生
