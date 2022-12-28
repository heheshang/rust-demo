// 泛型

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("{}, {}", p.x(), p.y());

    let p = Point { x: 5.0, y: 10.0 };
    let res = p.distance_from_origin();
    println!("{}", res);
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
