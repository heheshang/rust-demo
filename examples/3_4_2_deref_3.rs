// 解引用

fn main() {
    let s = String::from("Hello, world!");
    display(&s);
}

fn display(s: &str) {
    println!("{}", s);
}
