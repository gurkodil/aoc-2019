use std::cmp::Ordering;
use std::f32::consts::{FRAC_PI_2, PI};
use std::ops::Sub;

type Int = i16;

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: Int,
    pub y: Int,
}

fn gcd(a: Int, b: Int) -> Int {
    return if b == 0 { a } else { gcd(b, a % b) };
}

impl Point {
    pub fn normalize(&self) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        let den = gcd(x, y).abs();

        x /= den;
        y /= den;

        return Point { x, y };
    }

    fn angle(&self) -> f32 {
        // x = 0, y = 0: 0
        // x >= 0: arctan(y/x) -> [-pi/2, pi/2]
        // y >= 0: arctan(y/x) + pi -> (pi/2, pi]
        // y < 0: arctan(y/x) - pi -> (-pi, -pi/2)
        // Maps range between [0, 2 pi]
        ((self.y as f32).atan2(self.x as f32) + 5.0 * FRAC_PI_2) % (2.0 * PI)
    }

    pub fn mag(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}

impl Ord for Point {
    // Order points clockwise by angle
    fn cmp(&self, other: &Self) -> Ordering {
        self.angle().partial_cmp(&(other.angle())).unwrap()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
