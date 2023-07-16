use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Copy, Clone)]
pub struct Edge {
    dest: usize,
    weight: i32,
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.dest == other.dest
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dest.hash(state);
    }
}

impl Borrow<usize> for Edge {
    fn borrow(&self) -> &usize {
        &self.dest
    }
}

impl Edge {
    pub fn new(dest: usize, weight: i32) -> Self {
        Edge { dest, weight }
    }

    pub fn get_dest(&self) -> usize {
        self.dest
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }
}
