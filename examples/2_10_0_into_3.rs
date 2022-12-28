// trait object

fn main() {}

fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}

// fn do_stuff1<T>(value: &T) {
//     let cloned = <&T>::clone(value);
// }
