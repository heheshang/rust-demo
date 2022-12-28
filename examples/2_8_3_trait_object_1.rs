// trait object

pub trait Draw {
    fn draw(&self) -> String;
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for u8 {
    fn draw(&self) -> String {
        println!("draw u8 {}", *self);
        format!("draw u8 {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        println!("draw f64 {}", *self);
        format!("draw u64 {}", *self)
    }
}
// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}
fn draw2(x: &dyn Draw) {
    x.draw();
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("draw button {}", self.label)
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) -> String {
        println!("draw selectbox {}", self.width);
        format!("draw selectbox {}", self.width)
    }
}

// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }
// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
/*
    上面的 Screen 的列表中，存储了类型为 T 的元素，然后在 Screen 中使用特征约束让 T 实现了 Draw 特征，
    进而可以调用 draw 方法。

    但是这种写法限制了 Screen 实例的 Vec<T> 中的每个元素必须是 Button 类型或者全是 SelectBox 类型。
    如果只需要同质（相同类型）集合，更倾向于这种写法：使用泛型和 特征约束，
    因为实现更清晰，且性能更好(特征对象，需要在运行时从 vtable 动态查找需要调用的方法)。

*/

fn main() {
    let x = 1.1f64;
    let y = 2u8;

    draw1(Box::new(x));
    draw1(Box::new(y));

    draw2(&x);
    draw2(&y);
}
