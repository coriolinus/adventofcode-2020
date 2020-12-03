use crate::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    pub fn new(from: Point, to: Point) -> Line {
        Line { from, to }
    }

    pub fn manhattan_len(&self) -> i32 {
        (self.to - self.from).manhattan()
    }
}

// https://stackoverflow.com/a/1968345/504550
pub fn intersect(a: Line, b: Line) -> Option<Point> {
    let p0 = a.from;
    let p1 = a.to;
    let p2 = b.from;
    let p3 = b.to;

    let s1_x = (p1.x - p0.x) as f32;
    let s1_y = (p1.y - p0.y) as f32;
    let s2_x = (p3.x - p2.x) as f32;
    let s2_y = (p3.y - p2.y) as f32;

    let s =
        (-s1_y * (p0.x - p2.x) as f32 + s1_x * (p0.y - p2.y) as f32) / (-s2_x * s1_y + s1_x * s2_y);
    let t =
        (s2_x * (p0.y - p2.y) as f32 - s2_y * (p0.x - p2.x) as f32) / (-s2_x * s1_y + s1_x * s2_y);

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        // round the results so errors line up nicely
        Some(Point::new(
            p0.x + (t * s1_x).round() as i32,
            p0.y + (t * s1_y).round() as i32,
        ))
    } else {
        None
    }
}

pub fn intersections_naive(ap: &[Line], bp: &[Line]) -> Vec<Point> {
    let mut isects = Vec::new();
    for a in ap {
        for b in bp {
            if let Some(isect) = intersect(*a, *b) {
                isects.push(isect);
            }
        }
    }
    isects
}
