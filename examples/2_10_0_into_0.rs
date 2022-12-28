// trait object

/*
    强制类型转换的边角知识
    转换不具有传递性 就算 e as U1 as U2 是合法的，也不能说明 e as U2 是合法的（e 不能直接转换成 U2）。

*/
fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // if a < b {
    //     println!("a < b");
    // }

    if a < b as i32 {
        println!("a < b");
    }

    let a = i8::MAX;
    println!("a = {}", a);
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8; // 将字符'a'转换为整数，97

    println!("{},{},{}", a, b, c);

    let mut values = [1, 2];
    let p1 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p1 = 10;
        *p2 = 20;
    }
    assert_eq!(values, [10, 20]);
    println!("{:?}", values);
}
