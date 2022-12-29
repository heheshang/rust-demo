use std::fmt::Display;

fn main() {
    let mark_twain = "Mark Twain";
    print(mark_twain);
}
fn print<T: Display + 'static>(message: T) {
    println!("{}", message);
}
