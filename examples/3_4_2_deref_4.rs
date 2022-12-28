// 解引用
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    //这里我们使用了之前自定义的智能指针 MyBox，并将其通过连续的隐式转换变成 &str 类型：
    //首先 MyBox 被 Deref 成 String 类型，结果并不能满足 display 函数参数的要求，
    //编译器发现 String 还可以继续 Deref 成 &str，最终成功的匹配了函数参数
    let s = MyBox::new(String::from("hello"));
    display(&s);

    let s = MyBox::new(String::from("Rust"));
    display(&(*s)[..]);

    let s = MyBox::new(String::from("hello"));
    let s1: &str = &s;
    let s2 = s.to_string();
    display(s1);
    display(&s2);
}

fn display(s: &str) {
    println!("{}", s);
}
