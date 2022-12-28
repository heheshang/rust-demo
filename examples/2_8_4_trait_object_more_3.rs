// trait object

/*

在尖括号中，通过 as 关键字，我们向 Rust 编译器提供了类型注解，也就是 Animal 就是 Dog，而不是其他动物，
因此最终会调用 impl Animal for Dog 中的方法，获取到其它动物对狗宝宝的称呼：puppy。

言归正题，完全限定语法定义为：



<Type as Trait>::function(receiver_if_method, next_arg, ...);
上面定义中，第一个参数是方法接收器 receiver （三种 self），只有方法才拥有，例如关联函数就没有 receiver。

完全限定语法可以用于任何函数或方法调用，那么我们为何很少用到这个语法？
原因是 Rust 编译器能根据上下文自动推导出调用的路径，因此大多数时候，我们都无需使用完全限定语法。
只有当存在多个同名函数或方法，且 Rust 无法区分出你想调用的目标函数时，该用法才能真正有用武之地。

*/
fn main() {
    println!("A dog baby is called a {}", Dog::baby_name());

    // println!("A dog baby is called a {}", Animal::baby_name());

    println!("A dog baby is called a {}", <Dog as Animal>::baby_name());
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
