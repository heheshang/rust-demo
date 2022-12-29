fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");

        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    // println!("{:?}", i);
    // println!("{:?}", i.part)
}
/*
观察代码，可以看出结构体比它引用的字符串活得更久，引用字符串在内部语句块末尾 } 被释放后，
println! 依然在外面使用了该结构体，因此会导致无效的引用，不出所料，编译报错：

*/

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
