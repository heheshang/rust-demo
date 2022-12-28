// trait object

use std::ops::Deref;

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "[")?;

        // for (count, v) in self.0.iter().enumerate() {
        //     if count != 0 { write!(f, ", ")?; }
        //     write!(f, "{}", v)?;
        // }

        // write!(f, "]")

        write!(f, "[{}]", self.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
