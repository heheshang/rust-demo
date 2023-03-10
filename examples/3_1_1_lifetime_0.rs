// lifetime
/*
    需要注意的点如下：

    和泛型一样，使用生命周期参数，需要先声明 <'a>
    x、y 和返回值至少活得和 'a 一样久(因为返回值要么是 x，要么是 y)
    该函数签名表明对于某些生命周期 'a，函数的两个参数都至少跟 'a 活得一样久，同时函数的返回引用也至少跟 'a 活得一样久。实际上，这意味着返回值的生命周期与参数生命周期中的较小值一致：虽然两个参数的生命周期都是标注了 'a，但是实际上这两个参数的真实生命周期可能是不一样的(生命周期 'a 不代表生命周期等于 'a，而是大于等于 'a)。

    回忆下“鲁迅”说的话，再参考上面的内容，可以得出：在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。

    因此 longest 函数并不知道 x 和 y 具体会活多久，只要知道它们的作用域至少能持续 'a 这么长就行。

    当把具体的引用传给 longest 时，那生命周期 'a 的大小就是 x 和 y 的作用域的重合部分，换句话说，'a 的大小将等于 x 和 y 中较小的那个。由于返回值的生命周期也被标记为 'a，因此返回值的生命周期也是 x 和 y 中作用域较小的那个。

    说实话，这段文字我写的都快崩溃了，不知道你们读起来如何，实在***太绕了。。那就干脆用一个例子来解释吧：

*/
fn main() {
    let string1 = String::from("hello11");
    {
        let string2 = String::from("world");
        let result = largest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
    }
}
/*
    在上例中，string1 的作用域直到 main 函数的结束，而 string2 的作用域到内部花括号的结束 }，那么根据之前的理论，'a 是两者中作用域较小的那个，
    也就是 'a 的生命周期等于 string2 的生命周期，同理，由于函数返回的生命周期也是 'a，可以得出函数返回的生命周期也等于 string2 的生命周期。

    现在来验证下上面的结论：result 的生命周期等于参数中生命周期最小的，
    因此要等于 string2 的生命周期，也就是说，result 要活得和 string2 一样久，观察下代码的实现，可以发现这个结论是正确的！

*/

fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
鲁迅说过的话，总是值得重点标注，当你未来更加理解生命周期时，你才会发现这句话的精髓和重要！
现在先简单记住，标记的生命周期只是为了取悦编译器，让编译器不要难为我们，记住了吗？没记住，再回头看一遍，这对未来你遇到生命周期问题时会有很大的帮助！

在很多时候编译器是很聪明的，但是总有些时候，它会化身大聪明，自以为什么都很懂，然后去拒绝我们代码的执行，
此时，就需要我们通过生命周期标注来告诉这个大聪明：别自作聪明了，听我的就好。

例如一个变量，只能活一个花括号，那么就算你给它标注一个活全局的生命周期，它还是会在前面的花括号结束处被释放掉，并不会真的全局存活。

生命周期的语法也颇为与众不同，以 ' 开头，名称往往是一个单独的小写字母，大多数人都用 'a 来作为生命周期的名称。
如果是引用类型的参数，那么生命周期会位于引用符号 & 之后，并用一个空格来将生命周期和引用参数分隔开:





*/

/*
    一个生命周期标注，它自身并不具有什么意义，因为生命周期的作用就是告诉编译器多个引用之间的关系。
    例如，有一个函数，它的第一个参数 first 是一个指向 i32 类型的引用，具有生命周期 'a，
    该函数还有另一个参数 second，它也是指向 i32 类型的引用，并且同样具有生命周期 'a。此处生命周期标注仅仅说明，
    这两个参数 first 和 second 至少活得和'a 一样久，至于到底活多久或者哪个活得更久，抱歉我们都无法得知：

*/
