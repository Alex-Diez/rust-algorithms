pub use algorithms::union_find::{UnionFind, QuickFind, QuickUnion, WeightedQuickUnion, PathCompressionWeightedQuickUnion};

pub use expectest::prelude::{be_true, be_false, be_equal_to};

describe! union_find_tests {

    describe! quick_find_tests {

        before_each {
            let mut quick_find = QuickFind::new(10);
        }

        it "should root be point itself" {
            expect!(quick_find.find(1)).to(be_equal_to(1));
            expect!(quick_find.find(2)).to(be_equal_to(2));
        }

        it "should root be last connected point for multiple unions" {
            quick_find.union(1, 2);

            expect!(quick_find.find(1)).to(be_equal_to(2));

            quick_find.union(2, 3);

            expect!(quick_find.find(1)).to(be_equal_to(3));
            expect!(quick_find.find(2)).to(be_equal_to(3));

            quick_find.union(3, 4);

            expect!(quick_find.find(1)).to(be_equal_to(4));
            expect!(quick_find.find(2)).to(be_equal_to(4));
            expect!(quick_find.find(3)).to(be_equal_to(4));

            quick_find.union(4, 5);

            expect!(quick_find.find(1)).to(be_equal_to(5));
            expect!(quick_find.find(2)).to(be_equal_to(5));
            expect!(quick_find.find(3)).to(be_equal_to(5));
            expect!(quick_find.find(4)).to(be_equal_to(5));
        }

        it "should be connected united points" {
            quick_find.union(1, 2);

            expect!(quick_find.connected(1, 2)).to(be_true());
        }

        it "should not be connected not united points" {
            expect!(quick_find.connected(1, 3)).to(be_false());
        }

        it "should be connected having same connected point" {
            quick_find.union(1, 2);
            quick_find.union(2, 3);

            expect!(quick_find.connected(1, 3)).to(be_true());
        }
    }

    describe! quick_union_tests {

        before_each {
            let mut quick_union = QuickUnion::new(10);
        }

        it "should root be point itself" {
            expect!(quick_union.find(1)).to(be_equal_to(1));
            expect!(quick_union.find(2)).to(be_equal_to(2));
        }

        it "should be root last connecting point for multiple unions" {
            quick_union.union(1, 2);

            expect!(quick_union.find(1)).to(be_equal_to(2));

            quick_union.union(2, 3);

            expect!(quick_union.find(1)).to(be_equal_to(3));
            expect!(quick_union.find(2)).to(be_equal_to(3));

            quick_union.union(3, 4);

            expect!(quick_union.find(1)).to(be_equal_to(4));
            expect!(quick_union.find(2)).to(be_equal_to(4));
            expect!(quick_union.find(3)).to(be_equal_to(4));

            quick_union.union(4, 5);

            expect!(quick_union.find(1)).to(be_equal_to(5));
            expect!(quick_union.find(2)).to(be_equal_to(5));
            expect!(quick_union.find(3)).to(be_equal_to(5));
            expect!(quick_union.find(4)).to(be_equal_to(5));
        }

        it "should be connected united points" {
            quick_union.union(1, 2);

            expect!(quick_union.connected(1, 2)).to(be_true());
        }

        it "should not be connected not united points" {
            expect!(quick_union.connected(1, 3)).to(be_false());
        }

        it "should be connected having same united point" {
            quick_union.union(1, 2);
            quick_union.union(2, 3);

            expect!(quick_union.connected(1, 3)).to(be_true());
        }
    }

    describe! weighted_quick_union_tests {

        before_each {
            let mut weighted_quick_union = WeightedQuickUnion::new(10);
        }

        it "should root be point itself" {
            expect!(weighted_quick_union.find(1)).to(be_equal_to(1));
            expect!(weighted_quick_union.find(2)).to(be_equal_to(2));
        }

        it "should root be connected point" {
            weighted_quick_union.union(1, 2);

            expect!(weighted_quick_union.find(1)).to(be_equal_to(2));
        }

        it "should be connected united points" {
            weighted_quick_union.union(1, 2);

            expect!(weighted_quick_union.connected(1, 2)).to(be_true());
        }

        it "should not be connected not united points" {
            expect!(weighted_quick_union.connected(1, 3)).to(be_false());
        }

        it "should be connected having same united point" {
            weighted_quick_union.union(1, 2);
            weighted_quick_union.union(2, 3);

            expect!(weighted_quick_union.connected(1, 3)).to(be_true());
        }

        it "should root be a point with the biggest number of connection" {
            weighted_quick_union.union(1, 2);
            weighted_quick_union.union(2, 3);
            weighted_quick_union.union(2, 4);

            weighted_quick_union.union(5, 6);
            weighted_quick_union.union(6, 7);

            weighted_quick_union.union(2, 6);

            expect!(weighted_quick_union.find(6)).to(be_equal_to(2));
        }
    }

    describe! path_compression_weighted_quick_union_tests {

        before_each {
            let mut path_compression_weighted_quick_union = PathCompressionWeightedQuickUnion::new(10);
        }

        it "should root be point itself" {
            expect!(path_compression_weighted_quick_union.find(1)).to(be_equal_to(1));
            expect!(path_compression_weighted_quick_union.find(2)).to(be_equal_to(2));
        }

        it "should root be connected point" {
            path_compression_weighted_quick_union.union(1, 2);

            expect!(path_compression_weighted_quick_union.find(1)).to(be_equal_to(2));
        }

        it "should be connected united points" {
            path_compression_weighted_quick_union.union(1, 2);

            expect!(path_compression_weighted_quick_union.connected(1, 2)).to(be_true());
        }

        it "should not be connected not united points" {
            expect!(path_compression_weighted_quick_union.connected(1, 3)).to(be_false());
        }

        it "should be connected having same united point" {
            path_compression_weighted_quick_union.union(1, 2);
            path_compression_weighted_quick_union.union(2, 3);

            expect!(path_compression_weighted_quick_union.connected(1, 3)).to(be_true());
        }

        it "should root be a point with the biggest number of connection" {
            path_compression_weighted_quick_union.union(1, 2);
            path_compression_weighted_quick_union.union(2, 3);
            path_compression_weighted_quick_union.union(2, 4);

            path_compression_weighted_quick_union.union(5, 6);
            path_compression_weighted_quick_union.union(6, 7);

            path_compression_weighted_quick_union.union(2, 6);

            expect!(path_compression_weighted_quick_union.find(6)).to(be_equal_to(2));
        }
    }
}
