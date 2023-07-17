
#[derive(Debug, Copy, Clone)]
pub struct Node {
    id: usize,
    weight: f32,
    degree: u32,
}

impl Node {
    pub fn new(id: usize, weight: f32) -> Self {
        Node { id, weight, degree: 0 }
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn increment_degree(&mut self) {
        self.degree += 1;
    }

    pub fn get_degree(&self) -> u32 {
        self.degree
    }
}
