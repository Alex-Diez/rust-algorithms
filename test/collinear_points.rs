pub use std::cmp::Ordering;
pub use std::f32;

pub use algorithms::collinear_points::{Point, LineSegment, CollinearPoints};

pub use expectest::prelude::{be_true, be_equal_to};

describe! points_tests {

    it "should create a point" {
        Point::new(1, 1);
    }

    describe! point_comparison_by_x {

        before_each {
            const Y: i32 = 10;
            let small_x = Point::new(1, Y);
            let big_x = Point::new(5, Y);
        }

        it "should be greate than the point with less 'x'" {
            expect!(big_x > small_x).to(be_true());
        }

        it "should be greate or equal to the point with less 'x'" {
            expect!(big_x >= small_x).to(be_true());
        }

        it "should be less than the point with bigger 'x'" {
            expect!(small_x < big_x).to(be_true());
        }

        it "should be less or equal to the point with bigger 'x'" {
            expect!(small_x <= big_x).to(be_true());
        }
    }

    describe! point_comparison_by_y {

        before_each {
            const X: i32 = 10;
            let small_y = Point::new(X, 1);
            let big_y = Point::new(X, 5);
        }

        it "should be greate than the point with smaller 'y'" {
            expect!(big_y > small_y).to(be_true());
        }

        it "should be greate or equal to the point with smaller 'y'" {
            expect!(big_y >= small_y).to(be_true());
        }

        it "should be less than the point with bigger 'y'" {
            expect!(small_y < big_y).to(be_true());
        }

        it "should be less or equal to the point with bigger 'y'" {
            expect!(small_y <= big_y).to(be_true());
        }
    }

    describe! point_comparison_by_x_and_y {

        before_each {
            const BIG: i32 = 5;
            const SMALL: i32 = 1;
            let big_x_small_y = Point::new(BIG, SMALL);
            let small_x_big_y = Point::new(SMALL, BIG);
        }

        it "should be great than the point with smaller 'y' but bigger 'x'" {
            expect!(small_x_big_y > big_x_small_y);
        }

        it "should be great or equal to the point with smaller 'y' but bigger 'x'" {
            expect!(small_x_big_y >= big_x_small_y);
        }

        it "should be less than the point with bigger 'y' but smaller 'x'" {
            expect!(big_x_small_y < small_x_big_y);
        }

        it "should be less or equal to the point with bigger 'y' but smaller 'x'" {
            expect!(big_x_small_y <= small_x_big_y);
        }
    }

    it "should be equal to point with the same 'x' and 'y'" {
        let first = Point::new(1, 1);
        let second = Point::new(1, 1);

        expect!(first == second).to(be_true());
    }

    describe! slope {

        it "should calculate slope" {
            let x = 10;
            let y = 2;
            let x_that = 5;
            let y_that = 7;
            let point = Point::new(x, y);
            let that = Point::new(x_that, y_that);

            let slope = ((y - y_that) as f32) / ((x - x_that) as f32);

            expect!(point.slope_to(&that)).to(be_equal_to(slope));
        }

        it "should be equal to '0.0' if points are on the horizontal line" {
            let x = 10;
            let x_that = 5;
            let y = 10;
            let point = Point::new(x, y);
            let that = Point::new(x_that, y);

            expect!(point.slope_to(&that)).to(be_equal_to(0.0));
        }

        it "should be equal to positive infinity if points are on the vertical line" {
            let x = 10;
            let y = 5;
            let y_that = 10;
            let point = Point::new(x, y);
            let that = Point::new(x, y_that);

            expect!(point.slope_to(&that)).to(be_equal_to(f32::INFINITY));
        }

        it "should be equal to negative infinity if points have the same 'x' and 'y'" {
            let x = 10;
            let y = 5;
            let point = Point::new(x, y);
            let that = Point::new(x, y);

            expect!(point.slope_to(&that)).to(be_equal_to(f32::NEG_INFINITY));
        }
    }

    it "should be greater by slope with the point" {
        let point = Point::new(1, 1);
        let greater = Point::new(3, 4);
        let lesser = Point::new(5, 2);

        expect!(point.slope_order()(&greater, &lesser)).to(be_equal_to(Ordering::Greater));
    }

    it "should be lesser by slope with the point" {
        let point = Point::new(1, 1);
        let greater = Point::new(3, 4);
        let lesser = Point::new(5, 2);

        expect!(point.slope_order()(&lesser, &greater)).to(be_equal_to(Ordering::Less));
    }

    it "should be equal by slope with the point" {
        let point = Point::new(1, 1);
        let greater = Point::new(5, 5);
        let lesser = Point::new(3, 3);

        expect!(point.slope_order()(&greater, &lesser)).to(be_equal_to(Ordering::Equal));
    }
}

describe! line_segment_tests {

    it "should show a line segment direction" {
        let p = Point::new(1, 1);
        let q = Point::new(2, 2);
        let ls = LineSegment::new(p, q);

        expect!(ls.to_string()).to(be_equal_to("(1, 1) -> (2, 2)".to_owned()));
    }
}

describe! collinear_points_tests {

    it "should have 1 sigment" {
        let p = Point::new(1, 1);
        let q = Point::new(4, 4);
        let points = vec![p, Point::new(2, 2), Point::new(3, 3), q];
        let cp = CollinearPoints::new(&points);

        expect!(cp.number_of_segments()).to(be_equal_to(1));
        expect!(cp.segments()).to(be_equal_to(vec![LineSegment::new(p, q)]));
    }

    it "should not have any sigments if contain less than 3 points on the line" {
        let points = vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3), Point::new(1, 2), Point::new(1, 3), Point::new(2, 1), Point::new(3, 1)];
        let cp = CollinearPoints::new(&points);

        expect!(cp.number_of_segments()).to(be_equal_to(0));
        expect!(cp.segments()).to(be_equal_to(vec![]));
    }

    it "should have 3 sigments" {
        let p = Point::new(1, 1);
        let q1 = Point::new(1, 4);
        let q2 = Point::new(4, 1);
        let q3 = Point::new(4, 4);
        let points = vec![p, Point::new(2, 2), Point::new(3, 3), q1, Point::new(1, 2), Point::new(1, 3), q2, Point::new(2, 1), Point::new(3, 1), q3];
        let cp = CollinearPoints::new(&points);

        expect!(cp.number_of_segments()).to(be_equal_to(3));
        expect!(cp.segments()).to(be_equal_to(vec![LineSegment::new(p, q1), LineSegment::new(p, q2), LineSegment::new(p, q3)]));
    }

    it "should have 3 sigments if lines overlap" {
        let p = Point::new(1, 1);
        let p1 = Point::new(2, 2);
        let q1 = Point::new(4, 4);
        let q2 = Point::new(5, 5);
        let points = vec![p, p1, Point::new(3, 3), q1, q2];
        let cp = CollinearPoints::new(&points);

        expect!(cp.number_of_segments()).to(be_equal_to(3));
        expect!(cp.segments()).to(be_equal_to(vec![LineSegment::new(p, q1), LineSegment::new(p, q2), LineSegment::new(p1, q2)]));
    }
}
