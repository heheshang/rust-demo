use std::fmt::Display;

/*
    &'static 对于生命周期有着非常强的要求：一个引用必须要活得跟剩下的程序一样久，才能被标注为 &'static。

    对于字符串字面量来说，它直接被打包到二进制文件中，永远不会被 drop，因此它能跟程序活得一样久，
    自然它的生命周期是 'static。

    但是，&'static 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，
    还是要遵循相应的作用域规则 :

*/
fn main() {
    let mark_twain = "Mark Twain";
    print(&mark_twain);
}

fn print<T: Display + 'static>(message: &T) {
    println!("{}", message);
}
