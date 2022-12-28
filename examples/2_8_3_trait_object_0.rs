// trait object

fn main() {
    let objects = vec![UIObject::Button, UIObject::SelectBox];
    for object in objects {
        draw(object);
    }
}

#[derive(Debug)]
enum UIObject {
    Button,
    SelectBox,
}

fn draw(ui_object: UIObject) {
    println!("{:?}", ui_object);
}
