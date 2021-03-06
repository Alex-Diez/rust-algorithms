#![feature(test)]

extern crate test;
extern crate algorithms;

use std::fs::OpenOptions;
use std::io::Read;
use std::str::Lines;

use algorithms::union_find::{UnionFind, QuickFind, QuickUnion, WeightedQuickUnion, PathCompressionWeightedQuickUnion};

fn set_up_union_find(path: &str) -> (usize, Vec<(usize, usize)>) {
    let file_str = read_data_from_file(path);
    let mut lines = file_str.lines();
    let size = read_union_find_size(&mut lines);
    let data = populate_data(&mut lines);
    (size, data)
}

fn read_data_from_file(path: &str) -> String {
    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(_) => panic!("cant open to read file {}", path),
    };

    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(_) => {},
        Err(_) => panic!("cant read {} to string", path),
    }
    file_str
}

fn read_union_find_size(lines: &mut Lines) -> usize {
    match lines.next().map(|s| s.to_owned()) {
        Some(s) => match s.parse::<usize>() {
            Ok(l) => l,
            Err(_) => panic!("where is length?"),
        },
        None => panic!("where is length?"),
    }
}

fn populate_data(lines: &mut Lines) -> Vec<(usize, usize)> {
    let mut data = Vec::new();
    for l in lines {
        let mut split = l.split_whitespace();
        let p = match split.next().map(|s| s.to_owned()) {
            Some(s) => match s.parse::<usize>() {
                Ok(p) => p,
                Err(_) => panic!("where is row?"),
            },
            None => panic!("where is row?"),
        };
        let q = match split.next().map(|s| s.to_owned()) {
            Some(s) => match s.parse::<usize>() {
                Ok(q) => q,
                Err(_) => panic!("where is column?"),
            },
            None => panic!("where is column?"),
        };
        data.push((p, q));
    }
    data
}

#[bench]
fn quick_find_small_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            let mut quick_find = QuickFind::new(size);
            for &(p, q) in data.iter() {
                quick_find.union(p, q);
            }
        }
    )
}

#[bench]
fn quick_find_small_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            QuickFind::new(size)
        }
    )
}

#[bench]
fn quick_find_medium_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            let mut quick_find = QuickFind::new(size);
            for &(p, q) in data.iter() {
                quick_find.union(p, q);
            }
        }
    )
}

#[bench]
fn quick_find_medium_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            QuickFind::new(size)
        }
    )
}

#[bench]
fn quick_find_large_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            let mut quick_find = QuickFind::new(size);
            for &(p, q) in data.iter() {
                quick_find.union(p, q);
            }
        }
    )
}

#[bench]
fn quick_find_large_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            QuickFind::new(size)
        }
    )
}

#[bench]
fn quick_union_small_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            let mut quick_union = QuickUnion::new(size);
            for &(p, q) in data.iter() {
                quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn quick_union_small_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            QuickUnion::new(size)
        }
    )
}

#[bench]
fn quick_union_medium_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            let mut quick_union = QuickUnion::new(size);
            for &(p, q) in data.iter() {
                quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn quick_union_medium_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            QuickUnion::new(size)
        }
    )
}

#[bench]
fn quick_union_large_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            let mut quick_union = QuickUnion::new(size);
            for &(p, q) in data.iter() {
                quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn quick_union_large_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            QuickUnion::new(size)
        }
    )
}

#[bench]
fn weighted_quick_union_small_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            let mut weighted_quick_union = WeightedQuickUnion::new(size);
            for &(p, q) in data.iter() {
                weighted_quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn weighted_quick_union_small_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            WeightedQuickUnion::new(size)
        }
    )
}

#[bench]
fn weighted_quick_union_medium_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            let mut weighted_quick_union = WeightedQuickUnion::new(size);
            for &(p, q) in data.iter() {
                weighted_quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn weighted_quick_union_medium_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            WeightedQuickUnion::new(size)
        }
    )
}

#[bench]
fn weighted_quick_union_large_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            let mut weighted_quick_union = WeightedQuickUnion::new(size);
            for &(p, q) in data.iter() {
                weighted_quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn weighted_quick_union_large_creation(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            WeightedQuickUnion::new(size)
        }
    )
}

#[bench]
fn path_compression_weighted_quick_union_small_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            let mut path_compression_weighted_quick_union = PathCompressionWeightedQuickUnion::new(size);
            for &(p, q) in data.iter() {
                path_compression_weighted_quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn path_compression_weighted_quick_union_small_create(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_100");

    bench.iter(
        || {
            PathCompressionWeightedQuickUnion::new(size)
        }
    )
}

#[bench]
fn path_compression_weighted_quick_union_medium_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            let mut path_compression_weighted_quick_union = PathCompressionWeightedQuickUnion::new(size);
            for &(p, q) in data.iter() {
                path_compression_weighted_quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn path_compression_weighted_quick_union_medium_create(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_1000");

    bench.iter(
        || {
            PathCompressionWeightedQuickUnion::new(size)
        }
    )
}

#[bench]
fn path_compression_weighted_quick_union_large_population(bench: &mut test::Bencher) {
    let (size, data) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            let mut path_compression_weighted_quick_union = PathCompressionWeightedQuickUnion::new(size);
            for &(p, q) in data.iter() {
                path_compression_weighted_quick_union.union(p, q);
            }
        }
    )
}

#[bench]
fn path_compression_weighted_quick_union_large_create(bench: &mut test::Bencher) {
    let (size, _) = set_up_union_find("benches/union_find_10000");

    bench.iter(
        || {
            PathCompressionWeightedQuickUnion::new(size)
        }
    )
}
