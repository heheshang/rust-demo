// trait object
#![allow(unused)]

fn main() {
    fn foo() -> i32 {
        100
    }

    let pointer = foo as *const ();
    let function = unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };
    // assert_eq!(function(), 100);
    let ss = function();
    println!("{}", ss);
}
