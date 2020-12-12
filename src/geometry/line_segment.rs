use crate::geometry::{line::Line, Direction, Point};
use std::str::FromStr;

/// A `LineSegment` is a direction and a distance.
///
/// Classically, we'd call this a "vector", but that means
/// something different in programming.
#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub direction: Direction,
    pub distance: i32,
}

impl FromStr for LineSegment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("len < 2".into());
        }
        let direction = match s.to_ascii_lowercase().as_bytes()[0] {
            b'e' | b'r' => Direction::Right,
            b'w' | b'l' => Direction::Left,
            b'n' | b'u' => Direction::Up,
            b's' | b'd' => Direction::Down,
            unknown => return Err(format!("unknown direction: {}", unknown as char)),
        };
        let distance = s[1..]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        Ok(LineSegment {
            direction,
            distance,
        })
    }
}

pub fn follow(traces: &[LineSegment]) -> Vec<Line> {
    let mut cursor = Point::new(0, 0);
    let mut out = Vec::with_capacity(traces.len());
    for trace in traces {
        let prev = cursor;
        use Direction::*;
        let (val, mul) = match trace.direction {
            Right => (&mut cursor.x, 1),
            Left => (&mut cursor.x, -1),
            Up => (&mut cursor.y, 1),
            Down => (&mut cursor.y, -1),
        };
        *val += trace.distance * mul;
        out.push(Line::new(prev, cursor));
    }
    out
}
