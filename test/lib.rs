#![feature(const_fn, plugin)]
#![plugin(stainless)]

// for stainless
#![allow(unused_mut)]

extern crate algorithms;

#[macro_use]
extern crate expectest;

mod union_find;
mod percolation;
