// trait: a trait is a collection of methods that are defined for an unknown type: Self

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
/*
上面我们将 Summary 定义成了 pub 公开的。这样，如果他人想要使用我们的 Summary 特征，则可以引入到他们的包中，然后再进行实现。

关于特征实现与定义的位置，有一条非常重要的原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！ 例如我们可以为上面的 Post 类型实现标准库中的 Display 特征，这是因为 Post 类型定义在当前的作用域中。同时，我们也可以在当前包中为 String 类型实现 Summary 特征，因为 Summary 定义在当前作用域中。

但是你无法在当前作用域中，为 String 类型实现 Display 特征，因为它们俩都定义在标准库中，其定义所在的位置都不在当前作用域，跟你半毛钱关系都没有，看看就行了。

该规则被称为孤儿规则，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码。


*/
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub content: String,
    pub author: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{} by {} {}", self.title, self.author, self.content)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}
