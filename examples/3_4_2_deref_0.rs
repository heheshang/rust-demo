// 解引用
#[warn(dead_code)]
#[warn(unused_variables)]
#[derive(Debug)]
struct Person {
    #[warn(dead_code)]
    #[warn(unused_variables)]
    name: String,
    #[warn(unused_variables)]
    #[warn(dead_code)]
    age: u8,
}
#[warn(dead_code)]
#[warn(unused_variables)]
impl Person {
    #[warn(unused_variables)]
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
    #[warn(unused_variables)]
    fn display(self: &mut Person, age: u8) {
        let Person { name, age } = &self;
    }
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
