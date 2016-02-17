pub trait UnionFind {

    fn union(&mut self, p: usize, q: usize);

    fn connected(&self, p: usize, q: usize) -> bool;

    fn find(&self, p: usize) -> usize;
}

pub struct QuickFind {
    points: Vec<usize>
}

impl QuickFind {

    pub fn new(size: usize) -> QuickFind {
        let mut vec = Vec::with_capacity(size);
        for p in 0..size {
            vec.push(p);
        }
        QuickFind {
            points: vec
        }
    }
}

impl UnionFind for QuickFind {

    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.points[p];
        let q_root = self.points[q];
        for p in self.points.iter_mut() {
            if *p == p_root {
                *p = q_root;
            }
        }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn find(&self, p: usize) -> usize {
        self.points[p]
    }
}

pub struct QuickUnion {
    points: Vec<usize>
}

impl QuickUnion {

    pub fn new(size: usize) -> QuickUnion {
        let mut vec = Vec::with_capacity(size);
        for p in 0..size {
            vec.push(p);
        }
        QuickUnion {
            points: vec
        }
    }
}

impl UnionFind for QuickUnion {

    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        if p_root != q_root {
            self.points[p_root] = q_root;
        }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn find(&self, p: usize) -> usize {
        let mut point = p;
        while point != self.points[point] {
            point = self.points[point]
        }
        point
    }
}
