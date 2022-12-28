fn main() {
    let b = foo("World");
    println!("{}", b);

    let a = Box::new(1);
    println!("{}", a);
    // let b = a + 1;
    let b = *a + 1;
    println!("{}", b);
}
fn foo(s: &str) -> String {
    "Hello, ".to_string() + s
}
