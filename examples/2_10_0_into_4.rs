// trait object
#[allow(unused)]
use std::sync::Arc;

fn main() {
    // let foo = Container(Arc::new(1));
    // let bar = Container(Arc::new("Hello, world!"));
    // clone_container(&foo, &bar);
}
#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_container<T: Clone>(foo: &Container<i32>, bar: &Container<T>) {
    let _foo_cloned = foo.clone();
    let _bar_cloned = bar.clone();
}
