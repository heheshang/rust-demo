// static

fn main() {
    let mark_twain = "Mark Twain";
    print_author(mark_twain);
}
fn print_author(author: &'static str) {
    println!("Author: {}", author);
}
