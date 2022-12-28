/*


在之前，我们讲的都是不可变的 Deref 转换，实际上 Rust 还支持将一个可变的引用转换成另一个可变的引用以及将一个可变引用转换成不可变的引用，规则如下：

当 T: Deref<Target=U>，可以将 &T 转换成 &U，也就是我们之前看到的例子
当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
来看一个关于 DerefMut 的例子：
*/
struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
/*
    要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
    T: DerefMut<Target=U> 解读：将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，对应上例中，就是将 &mut MyBox<String> 转换为 &mut String
    对于上述三条规则中的第三条，它比另外两条稍微复杂了点：Rust 可以把可变引用隐式的转换成不可变引用，但反之则不行。

    如果从 Rust 的所有权和借用规则的角度考虑，当你拥有一个可变的引用，那该引用肯定是对应数据的唯一借用，
    那么此时将可变引用变成不可变引用并不会破坏借用规则；但是如果你拥有一个不可变引用，
    那同时可能还存在其它几个不可变的引用，如果此时将其中一个不可变引用转换成可变引用，
    就变成了可变引用与不可变引用的共存，最终破坏了借用规则。
*/
fn main() {
    let mut s = MyBox::new(String::from("hello"));
    display(&mut s);
}

fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}
