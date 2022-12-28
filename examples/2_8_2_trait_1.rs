// 借用

fn main() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from ...)")
    }
}

pub struct Post {
    pub title: String,
    pub content: String,
    pub author: String,
}
impl Summary for Post {}

pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}
