// 借用

fn main() {
    // let s = String::from("hello");
    // change(&s);
    let mut s1 = String::from("hello");
    change1(&mut s1);
}

// fn change(some_thing: &String) {
// some_thing.push_str(", world");
// }

/*

    可变引用同时只能存在一个
    不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用：

*/
fn change1(some_thing: &mut String) {
    some_thing.push_str(", world");
}
