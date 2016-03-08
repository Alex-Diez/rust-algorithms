use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use fnv::FnvHasher;

use generator::{Generator, DefaultGenerator};

pub trait UnionFind {

    fn union(&mut self, p: usize, q: usize);

    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn find(&mut self, p: usize) -> usize;
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

    fn find(&mut self, p: usize) -> usize {
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

    fn find(&mut self, p: usize) -> usize {
        let mut point = p;
        while point != self.points[point] {
            point = self.points[point]
        }
        point
    }
}

pub struct WeightedQuickUnion {
    points: Vec<usize>,
    sizes: Vec<usize>
}

impl WeightedQuickUnion {

    pub fn new(size: usize) -> WeightedQuickUnion {
        let mut vec = Vec::with_capacity(size);
        let mut sizes = Vec::with_capacity(size);
        for p in 0..size {
            vec.push(p);
            sizes.push(1);
        }
        WeightedQuickUnion {
            points: vec,
            sizes: sizes
        }
    }
}

impl UnionFind for WeightedQuickUnion {

    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        if self.sizes[p_root] <= self.sizes[q_root] {
            self.points[p_root] = q_root;
            self.sizes[q_root] += self.sizes[p_root];
        }
        else {
            self.points[q_root] = p_root;
            self.sizes[p_root] += self.sizes[q_root];
        }
    }

    fn find(&mut self, p: usize) -> usize {
        let mut point = p;
        while point != self.points[point] {
            point = self.points[point];
        }
        point
    }
}

type FasterHasher = BuildHasherDefault<FnvHasher>;

pub struct PathCompressionWeightedQuickUnion {
    points: HashMap<usize, usize, FasterHasher>,
    sizes: HashMap<usize, usize, FasterHasher>
}

impl PathCompressionWeightedQuickUnion {

    pub fn new(size: usize) -> PathCompressionWeightedQuickUnion {
        let mut generator = DefaultGenerator::new();
        PathCompressionWeightedQuickUnion::with_generator(size, &mut generator)
    }

    pub fn with_generator<G: Generator>(size: usize, generator: &mut G) -> PathCompressionWeightedQuickUnion {
        let mut sizes = HashMap::with_capacity_and_hasher(size, FasterHasher::default());
        let mut points = HashMap::with_capacity_and_hasher(size, FasterHasher::default());
        for _ in (0..).take(size) {
            let n = generator.next();
            points.insert(n, n);
            sizes.insert(n, 1);
        }
        PathCompressionWeightedQuickUnion {
            points: points,
            sizes: sizes
        }
    }
}

impl UnionFind for PathCompressionWeightedQuickUnion {

    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        if self.sizes[&p_root] <= self.sizes[&q_root] {
            let p_entry = self.points.entry(p_root).or_insert(p_root);
            *p_entry = q_root;
            let p_size = self.sizes[&p_root];
            let q_entry = self.sizes.entry(q_root).or_insert(1);
            *q_entry += p_size;
        }
        else {
            let q_entry = self.points.entry(q_root).or_insert(q_root);
            *q_entry = p_root;
            let q_size = self.sizes[&q_root];
            let p_entry = self.sizes.entry(p_root).or_insert(1);
            *p_entry += q_size;
        }
    }

    fn find(&mut self, p: usize) -> usize {
        let mut point = p;
        while point != self.points[&point] {
            let p = point;
            let parent = self.points[&(self.points[&point])];
            point = parent;
            self.points.insert(p, parent);
            let p_entry = self.points.entry(p).or_insert(p);
            *p_entry = parent;
        }
        point
    }
}
