/*
比枚举实现要稍微复杂一些，我们为 V4 和 V6 都实现了特征 IpAddr，然后将它俩的实例用 Box::new 包裹后，存在了数组 v 中，需要注意的是，这里必须手动地指定类型：Vec<Box<dyn IpAddr>>，表示数组 v 存储的是特征 IpAddr 的对象，这样就实现了在数组中存储不同的类型。

在实际使用场景中，特征对象数组要比枚举数组常见很多，主要原因在于特征对象非常灵活，而编译器对枚举的限制较多，且无法动态增加类型。

最后，如果你想要了解 Vector 更多的用法，请参见本书的标准库解析章节：Vector常用方法

*/
fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in &v {
        ip.display();
    }
}
trait IpAddr {
    fn display(&self);
}

struct V4(String);
struct V6(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("V4: {}", self.0);
    }
}
impl IpAddr for V6 {
    fn display(&self) {
        println!("V6: {}", self.0);
    }
}
