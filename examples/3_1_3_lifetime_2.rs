fn main() {
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);

    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );

    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
    // println!("{}", message);
}

fn get_memory_location() -> (usize, usize) {
    let string = "Hello, world!";
    let pointer = string.as_ptr() as usize;

    let length = string.len();
    (pointer, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe {
        let slice = std::slice::from_raw_parts(pointer as *const u8, length);
        std::str::from_utf8_unchecked(slice)
    }
}
