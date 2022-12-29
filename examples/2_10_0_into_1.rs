// trait object

/*
    强制类型转换的边角知识
    转换不具有传递性 就算 e as U1 as U2 是合法的，也不能说明 e as U2 是合法的（e 不能直接转换成 U2）。

*/
fn main() {
    let a: u8 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("a < b");
    }

    let b: i16 = 10000;
    let b_: u8 = match b.try_into() {
        Ok(v) => {
            println!("v = {}", v);
            v
        }
        Err(e) => {
            println!("error: {}", e);
            0
        }
    };
    println!("b_ = {}", b_);
    /*
    强制类型转换
    在某些情况下，类型是可以进行隐式强制转换的，虽然这些转换弱化了 Rust 的类型系统，
    但是它们的存在是为了让 Rust 在大多数场景可以工作(说白了，帮助用户省事)，而不是报各种类型上的编译错误。

    首先，在匹配特征时，不会做任何强制转换(除了方法)。一个类型 T 可以强制转换为 U，
    不代表 impl T 可以强制转换为 impl U，例如下面的代码就无法通过编译检查：

    */
    // let t = &mut 0;
    // foo(t);
}

struct Foo {
    x: i32,
    y: i32,
}
struct Bar {
    a: i32,
    b: i32,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}

trait Trait {}
fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}
