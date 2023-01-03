fn main() {
    let array: [String; 8] = core::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:?}", array);
}
