use std::collections::HashSet;
use std::cmp::Ordering;
use std::f32;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
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

impl fmt::Display for Point {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
pub struct LineSegment {
    p: Point,
    q: Point
}

impl LineSegment {

    pub fn new(p: Point, q: Point) -> LineSegment {
        LineSegment {
            p: p,
            q: q
        }
    }
}

impl fmt::Display for LineSegment {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.p, self.q)
    }
}

pub struct CollinearPoints {
    segments: HashSet<LineSegment>
}

impl CollinearPoints {

    pub fn new(points: &Vec<Point>) -> CollinearPoints {
        let mut points = points.iter().cloned().collect::<Vec<Point>>();
        points.sort();
        let mut segments = HashSet::new();
        for i in 0..points.len() - 3 {
            let i_point = points[i];
            for j in (i + 1)..points.len() - 2 {
                let j_point = points[j];
                for k in (j + 1)..points.len() - 1 {
                    let k_point = points[k];
                    for l in (k + 1)..points.len() {
                        let l_point = points[l];
                        let slope_order = i_point.slope_order();
                        if slope_order(&j_point, &k_point) == Ordering::Equal
                                && slope_order(&k_point, &l_point) == Ordering::Equal
                                && slope_order(&l_point, &j_point) == Ordering::Equal {
                            segments.insert(LineSegment::new(i_point, l_point));
                        }
                    }
                }
            }
        }
        CollinearPoints {
            segments: segments
        }
    }

    pub fn number_of_segments(&self) -> usize {
        self.segments.len()
    }

    pub fn segments(&self) -> Vec<LineSegment> {
        let mut ret = self.segments.iter().cloned().collect::<Vec<LineSegment>>();
        ret.sort();
        ret
    }
}
