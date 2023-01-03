fn main() {
    let v: Vec<i32> = vec1![1, 2, 3];
    println!("Hello, world!");
    println!("{:?}", v);
}

#[macro_export]
macro_rules! vec1 {
    ($($x:expr),*) => {
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
            )*
            tmp_vec
        }
    };
}
