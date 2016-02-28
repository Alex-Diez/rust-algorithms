#![feature(test)]

extern crate test;
extern crate algorithms;

use std::fs::OpenOptions;
use std::io::Read;
use std::str::Lines;

use algorithms::percolation::{Percolation, BruteForcePercolation, UnionFindPercolation, HackUnionFindPercolation};

fn set_up_percolation(path: &str) -> (usize, Vec<(usize, usize)>) {
    let file_str = read_data_from_file(path);
    let mut lines = file_str.lines();
    let side_size = read_percolation_side_size(&mut lines);
    let data = populate_data(&mut lines);
    (side_size, data)
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

fn read_percolation_side_size(lines: &mut Lines) -> usize {
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
        let r = match split.next().map(|s| s.to_owned()) {
            Some(s) => match s.parse::<usize>() {
                Ok(r) => r,
                Err(_) => panic!("where is row?"),
            },
            None => panic!("where is row?"),
        };
        let c = match split.next().map(|s| s.to_owned()) {
            Some(s) => match s.parse::<usize>() {
                Ok(c) => c,
                Err(_) => panic!("where is column?"),
            },
            None => panic!("where is column?"),
        };
        data.push((r, c));
    }
    data
}

#[bench]
fn brute_force_percolation_small(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_10x10");
    let mut percolation = BruteForcePercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn brute_force_percolation_medium(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_100x100");
    let mut percolation = BruteForcePercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn brute_force_percolation_large(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_500x500");
    let mut percolation = BruteForcePercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn union_find_percolation_small(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_10x10");
    let mut percolation = UnionFindPercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn union_find_percolation_medium(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_100x100");
    let mut percolation = UnionFindPercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn union_find_percolation_large(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_500x500");
    let mut percolation = UnionFindPercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn hack_union_find_percolation_small(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_10x10");
    let mut percolation = HackUnionFindPercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn hack_union_find_percolation_medium(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_100x100");
    let mut percolation = HackUnionFindPercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}

#[bench]
fn hack_union_find_percolation_large(bench: &mut test::Bencher) {
    let (side_size, data) = set_up_percolation("benches/percolation_500x500");
    let mut percolation = HackUnionFindPercolation::new(side_size);

    bench.iter(
        || {
            for &(r, c) in data.iter() {
                percolation.open(r, c);
            }
        }
    )
}
