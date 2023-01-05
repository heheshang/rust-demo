use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams_list = vec![(String::from("Blue"), 10), (String::from("Yellow"), 50)];

    let teams_map = teams_list.into_iter().collect::<HashMap<_, _>>();
    println!("{:?}", teams_map);
}
