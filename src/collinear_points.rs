use std::cmp::{PartialOrd, Ordering};
use std::f32;

#[derive(PartialEq)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {

    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn slope_to(&self, that: &Point) -> f32 {
        if self == that { f32::NEG_INFINITY }
        else if self.x == that.x { f32::INFINITY }
        else { ((self.y - that.y) as f32) / ((self.x - that.x) as f32) }
    }

    pub fn slope_order(&self) -> Box<Fn(&Point, &Point) -> Ordering> {
        let Point {x, y} = *self;
        Box::new(
            move |p: &Point, q: &Point| {
                let point = Point::new(x, y);
                let p_slope = point.slope_to(p);
                let q_slope = point.slope_to(q);
                if p_slope > q_slope { Ordering::Greater }
                else if p_slope < q_slope { Ordering::Less }
                else { Ordering::Equal }
            }
        )
    }
}

impl PartialOrd for Point {

    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        if self.x > other.x { Some(Ordering::Greater) }
        else if self.x < other.x { Some(Ordering::Less) }
        else if self.y > other.y { Some(Ordering::Greater) }
        else if self.y < other.y { Some(Ordering::Less) }
        else { Some(Ordering::Equal) }
    }
}
