// trait 调用方法需要引入特征

fn main() {
    let a: i32 = 1;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("a < b");
    }
}
