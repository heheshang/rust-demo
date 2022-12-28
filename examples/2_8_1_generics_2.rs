// 借用

fn main() {
    // let p = Point { x: 1, y: 2.1 };
    let p = Point { x: 1, y: 2 };
    println!("{}, {}", p.x, p.y);
    let p1 = Point1 { x: 1, y: 2.1 };
    println!("{}, {}", p1.x, p1.y);
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point1<T, U> {
    x: T,
    y: U,
}
#[warn(dead_code)]
enum Option<T> {
    Some(T),
    None,
}
#[warn(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
