// 借用

fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    /*
    cannot borrow `s` as mutable more than once at a time
    second mutable borrow occurs hererustcClick for full compiler diagnostic
    232_borrow_3.rs(5, 14): first mutable borrow occurs here
    232_borrow_3.rs(7, 24): first borrow later used here

    */
    // let r2 = &mut s;
    println!("{}", r1);
}
/*

    这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：

    两个或更多的指针同时访问同一数据
    至少有一个指针被用来写入数据
    没有同步数据访问的机制
    数据竞争会导致未定义行为，这种行为很可能超出我们的预期，难以在运行时追踪，并且难以诊断和修复。
    而 Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

    很多时候，大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域：

*/
