// trait

use std::fmt::Display;

// #[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(1, 2);
    // println!("{:?}", pair);
    pair.cmp_display();
    let sum = return_summarize();
    println!("{}", sum.summarize());
}

fn return_summarize() -> impl Summary {
    Weibo {
        username: String::from("username"),
        content: String::from("content"),
    }
}
/*

以上的代码就无法通过编译，因为它返回了两个不同的类型 Post 和 Weibo。


`if` and `else` have incompatible types
expected struct `Post`, found struct `Weibo`
报错提示我们 if 和 else 返回了不同的类型。如果想要实现返回不同的类型，需要使用下一章节中的特征对象。

*/
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Box::new(Weibo {
//             username: String::from("username"),
//             content: String::from("content"),
//         })
//     } else {
//         Box::new(Post {
//             title: String::from("title"),
//             content: String::from("content"),
//             author: String::from("author"),
//         })
//     }
// }

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

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
