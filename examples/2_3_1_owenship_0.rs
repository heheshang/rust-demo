// 所有权
/*
    代码背后的逻辑很简单, 将 5 绑定到变量 x；接着拷贝 x 的值赋给 y，最终 x 和 y 都等于 5，
    因为整数是 Rust 基本数据类型，是固定大小的简单值，因此这两个值都是通过自动拷贝的方式来赋值的，
    都被存在栈中，完全无需在堆上分配内存。

*/
fn main() {
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);
    /*
    总之 String 类型指向了一个堆上的空间，这里存储着它的真实数据, 下面对上面代码中的 let s2 = s1 分成两种情况讨论：

    拷贝 String 和存储在堆上的字节数组 如果该语句是拷贝所有数据(深拷贝)，那么无论是 String 本身还是底层的堆上数据，
    都会被全部拷贝，这对于性能而言会造成非常大的影响

    只拷贝 String 本身 这样的拷贝非常快，因为在 64 位机器上就拷贝了 8字节的指针、8字节的长度、8字节的容量，总计 24 字节，
    但是带来了新的问题，还记得我们之前提到的所有权规则吧？其中有一条就是：一个值只允许有一个所有者，
    而现在这个值（堆上的真实字符串数据）有了两个所有者：s1 和 s2。

    当变量离开作用域后，Rust 会自动调用 drop 函数并清理变量的堆内存。不过由于两个 String 变量指向了同一位置。
    这就有了一个问题：当 s1 和 s2 离开作用域，它们都会尝试释放相同的内存。这是一个叫做 二次释放（double free） 的错误，
    也是之前提到过的内存安全性 BUG 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

    因此，Rust 这样解决问题：当 s1 赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，
    这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。
    */
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
}
