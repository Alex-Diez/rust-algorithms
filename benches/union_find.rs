#![feature(test)]

extern crate test;
extern crate rand;
extern crate algorithms;

use rand::distributions::{IndependentSample, Range};

use algorithms::union_find::{UnionFind, QuickFind, QuickUnion, WeightedQuickUnion};

const SIZE: usize = 100;

struct Connection(usize, usize);

fn generate_rand_data() -> Vec<Connection> {
    let mut rand_data = Vec::with_capacity(SIZE);
    let between: Range<usize> = Range::new(1, SIZE);
    let mut rng = rand::thread_rng();
    for _ in 0..SIZE {
        let p = between.ind_sample(&mut rng);
        let q = between.ind_sample(&mut rng);
        rand_data.push(Connection(p, q));
    }
    rand_data
}

#[bench]
fn quick_find_random(bench: &mut test::Bencher) {
    let rand_data = generate_rand_data();
    let mut quick_find = QuickFind::new(SIZE);

    bench.iter(
        || {
            for i in rand_data.iter() {
                quick_find.union(i.0, i.1);
            }
        }
    )
}

#[bench]
fn quick_union_random(bench: &mut test::Bencher) {
    let rand_data = generate_rand_data();
    let mut quick_union = QuickUnion::new(SIZE);

    bench.iter(
        || {
            for i in rand_data.iter() {
                quick_union.union(i.0, i.1);
            }
        }
    )
}

#[bench]
fn weighted_quick_union_random(bench: &mut test::Bencher) {
    let rand_data = generate_rand_data();
    let mut weighted_quick_union = WeightedQuickUnion::new(SIZE);

    bench.iter(
        || {
            for i in rand_data.iter() {
                weighted_quick_union.union(i.0, i.1);
            }
        }
    )
}
