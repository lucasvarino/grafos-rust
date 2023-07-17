#[derive(Debug)]
pub struct Node {
    id: i32,
    weight: f32,
}

impl Node {
    pub fn new(id: i32, weight: f32) -> Self {
        Node { id, weight }
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
