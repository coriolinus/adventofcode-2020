use crate::geometry::{line::Line, Direction, Point};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Trace {
    direction: Direction,
    distance: i32,
}

impl FromStr for Trace {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("len < 2".into());
        }
        let direction = match s.as_bytes()[0] {
            b'R' | b'r' => Direction::Right,
            b'L' | b'l' => Direction::Left,
            b'U' | b'u' => Direction::Up,
            b'D' | b'd' => Direction::Down,
            unknown => return Err(format!("unknown direction: {}", unknown as char)),
        };
        let distance = s[1..]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        Ok(Trace {
            direction,
            distance,
        })
    }
}

pub fn follow(traces: &[Trace]) -> Vec<Line> {
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
