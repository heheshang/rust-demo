use std::fmt::Debug;
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
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    // --snip--
    1
}

fn some_function1<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // --snip--
    1
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
