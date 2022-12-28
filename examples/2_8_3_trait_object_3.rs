// trait object

fn main() {
    let button = Button;
    let newb = button.draw();
    print!("{:?}", newb);
}

trait Draw {
    fn draw(&self) -> Self;
}

#[derive(Clone, Debug)]
struct Button;
impl Draw for Button {
    fn draw(&self) -> Self {
        println!("draw button");
        self.clone()
    }
}
/*

    不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，
    它的对象才是安全的：

    方法的返回类型不能是 Self
    方法没有任何泛型参数
    对象安全对于特征对象是必须的，因为一旦有了特征对象，就不再需要知道实现该特征的具体类型是什么了。
    如果特征方法返回了具体的 Self 类型，但是特征对象忘记了其真正的类型，那这个 Self 就非常尴尬，
    因为没人知道它是谁了。但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：
    此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，
    故而无从得知放入泛型参数类型到底是什么。
*/
// pub struct Screen {
//     pub components: Vec<Box<dyn Clone>>,
// }
