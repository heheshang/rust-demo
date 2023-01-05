fn main() {
    let name = String::from("rust");
    let age = 18;
    let mut map = std::collections::HashMap::new();
    // map.insert(name, age);
    map.insert(&name, &age);
    println!("{:?}", map);
    // std::mem::drop(name);
    println!("{:?}", name);
    println!("{:?}", age);
}
