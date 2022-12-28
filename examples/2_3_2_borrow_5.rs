// 借用
/*

    cannot borrow `s` as mutable because it is also borrowed as immutable
    mutable borrow occurs hererustcClick for full compiler diagnostic
    232_borrow_5.rs(5, 14): immutable borrow occurs here
    232_borrow_5.rs(8, 32): immutable borrow later used here

*/
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中 r1 和 r2 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r3 = &mut s;
    println!("r3 {}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束
