// trait object

use std::fmt;

fn main() {
    let mut c = Conuter { id: 0 };
    c.next();
}

struct Conuter {
    id: u32,
}

impl Iterator for Conuter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

// pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
//     type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
//     fn is_null(&self) -> bool;
// }
