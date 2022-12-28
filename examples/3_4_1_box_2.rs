use std::vec;

fn main() {
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
    for elem in elems {
        elem.draw();
    }

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    println!("sum {}", sum);
}

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button {}", self.id);
    }
}

struct Select {
    id: u32,
}
impl Draw for Select {
    fn draw(&self) {
        println!("draw select {}", self.id);
    }
}
