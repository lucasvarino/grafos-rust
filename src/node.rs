#[derive(Debug)]
pub struct Node {
    id: usize,
    weight: f32,
}

impl Node {
    pub fn new(id: usize, weight: f32) -> Self {
        Node { id, weight }
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
