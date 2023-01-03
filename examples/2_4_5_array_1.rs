fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();
    // 控制台输入
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // 字符串转数字
    let index: usize = index.trim().parse().expect("Please type a number!");

    let element = a[index];
    println!("The value of the element is: {}", element);
}
