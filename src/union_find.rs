use std::vec::Vec;

pub struct UnionFind {
    points: Vec<usize>
}

impl UnionFind {

    pub fn new(size: usize) -> UnionFind {
        let mut vec = Vec::with_capacity(size);
        for p in 0..size {
            vec.push(p);
        }
        UnionFind {
            points: vec
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let p_root = self.points[p];
        let q_root = self.points[q];
        for p in self.points.iter_mut() {
            if *p == p_root {
                *p = q_root;
            }
        }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn find(&self, p: usize) -> usize {
        self.points[p]
    }
}
