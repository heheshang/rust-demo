// 泛型

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("{}, {}", p3.x, p3.y);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    /*
    这个例子中，T,U 是定义在结构体 Point 上的泛型参数，V,W 是单独定义在方法 mixup 上的泛型参数，
    它们并不冲突，说白了，你可以理解为，一个是结构体泛型，一个是函数泛型。

    */
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
