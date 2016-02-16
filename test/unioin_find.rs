#![allow(unused_mut)]

pub use algorithms::union_find::UnionFind;

pub use expectest::prelude::{be_true, be_false, be_equal_to};

describe! union_find_tests {

    before_each {
        let mut union_find = UnionFind::new(10);
    }

    it "should root be point itself" {
        expect!(union_find.find(1)).to(be_equal_to(1));
        expect!(union_find.find(2)).to(be_equal_to(2));
    }

    it "should root be connected point" {
        union_find.union(1, 2);

        expect!(union_find.find(1)).to(be_equal_to(2));
    }

    it "should be connected united points" {
        union_find.union(1, 2);

        expect!(union_find.connected(1, 2)).to(be_true());
    }

    it "should not be connected not united points" {
        expect!(union_find.connected(1, 3)).to(be_false());
    }

    it "should be connected having same connected point" {
        union_find.union(1, 2);
        union_find.union(2, 3);

        expect!(union_find.connected(1, 3)).to(be_true());
    }
}
