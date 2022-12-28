// 借用

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let res = largest(&number_list);
    println!("{}", res);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let res = largest(&char_list);
    println!("{}", res);
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
