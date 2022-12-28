use std::fmt::Display;

// trait
/*

    weibo.summarize() 会先调用 Summary 特征默认实现的 summarize 方法，
    通过该方法进而调用 Weibo 为 Summary 实现的 summarize_author 方法，
    最终输出：1 new weibo: (Read more from @horse_ebooks...)。
*/
fn main() {
    let post = Post {
        title: String::from("title"),
        content: String::from("content"),
        author: String::from("author"),
    };
    println!("{}", post.summarize());
    let weibo = Weibo {
        username: String::from("username"),
        content: String::from("content"),
    };
    println!("{}", weibo.summarize());
    notify(&weibo);
    notify(&post);
    notify2(&post, &post);
    notify3(&weibo);
    notify3(&post);
    /*

          Compiling rust-demo v0.1.0 (/opt/soft/rust-demo)
    error[E0308]: mismatched types
      --> examples/282_trait_3.rs:23:20
       |
    23 |     notify2(&post, &weibo);
       |     -------        ^^^^^^ expected struct `Post`, found struct `Weibo`
       |     |
       |     arguments to this function are incorrect
       |
       = note: expected reference `&Post`
                  found reference `&Weibo`
    note: function defined here
      --> examples/282_trait_3.rs:34:8
       |
    34 | pub fn notify2<T: Summary>(item: &T, item2: &T) {
       |        ^^^^^^^                       ---------

    For more information about this error, try `rustc --explain E0308`.

        */
    // notify2(&post, &weibo);
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item);
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item);
}

//特征绑定
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify2<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item2.summarize());
}
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
impl Display for Weibo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1 new weibo: {}", self.content)
    }
}

pub struct Post {
    pub title: String,
    pub content: String,
    pub author: String,
}
impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1 new post: {}", self.content)
    }
}
