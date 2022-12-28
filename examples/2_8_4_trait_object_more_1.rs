// trait object

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;
    fn add(self, other: Millimeters) -> Meters {
        Meters(self.0 + (other.0 / 1000))
    }
}
/*

    这里，是进行 Millimeters + Meters 两种数据类型的 + 操作，因此此时不能再使用默认的 RHS，否则就会变成 Millimeters + Millimeters 的形式。使用 Add<Meters> 可以将 RHS 指定为 Meters，那么 fn add(self, rhs: RHS) 自然而言的变成了 Millimeters 和 Meters 的相加。

    默认类型参数主要用于两个方面：

    减少实现的样板代码
    扩展类型但是无需大幅修改现有的代码
    之前的例子就是第一点，虽然效果也就那样。在 + 左右两边都是同样类型时，只需要 impl Add 即可，否则你需要 impl Add<SOME_TYPE>，嗯，会多写几个字:)

    对于第二点，也很好理解，如果你在一个复杂类型的基础上，新引入一个泛型参数，可能需要修改很多地方，但是如果新引入的泛型参数有了默认类型，情况就会好很多，添加泛型参数后，使用这个类型的代码需要逐个在类型提示部分添加泛型参数，就很麻烦；但是有了默认参数（且默认参数取之前的实现里假设的值的情况下）之后，原有的使用这个类型的代码就不需要做改动了。

    归根到底，默认泛型参数，是有用的，但是大多数情况下，咱们确实用不到，当需要用到时，大家再回头来查阅本章即可，手上有剑，心中不慌。

*/

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("p3 = {:?}", p3);
}
