#[derive(Debug)]
pub struct Node {
    id: i32,
    weight: f32,
}

impl Node {
    pub fn new(id: i32, weight: f32) -> Self {
        Node { id, weight }
    }
}
