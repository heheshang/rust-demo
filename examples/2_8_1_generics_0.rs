// 借用

fn main() {
    println!("{}", add(1i8, 2i8));
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
