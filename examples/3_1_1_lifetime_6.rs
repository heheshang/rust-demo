fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
    println!("{:?}", i.part);

    i.announce_and_return_part("hello");
    i.announce_and_return_part1("world");
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// impl ImportantExcerpt<'_> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
/*

就关键点稍微解释下：

'a: 'b，是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
可以把 'a 和 'b 都在同一个地方声明（如上），或者分开声明但通过 where 'a: 'b 约束生命周期关系，如下：
*/
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part1<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}
