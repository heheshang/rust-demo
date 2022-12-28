// trait

use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add + std::ops::Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));
}
