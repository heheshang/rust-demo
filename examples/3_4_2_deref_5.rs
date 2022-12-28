// 解引用

use std::rc::Rc;

fn main() {
    let owned = "Hello, world!".to_string();
    display(&owned);

    let owned = "Hello, world!".to_string();
    let counted = Rc::new(owned);
    display(&counted);

    let f = &&Foo;
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&f).foo();
}

fn display(s: &str) {
    println!("{}", s);
}
struct Foo;
impl Foo {
    fn foo(&self) {
        println!("Foo");
    }
}
