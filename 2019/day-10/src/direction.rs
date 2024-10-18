use std::{cmp::Ordering, ops::Mul};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Direction {
    pub x: isize,
    pub y: isize,
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Mul<(isize, isize)> for Direction {
    type Output = isize;

    fn mul(self, rhs: (isize, isize)) -> Self::Output {
        self.x * rhs.0 + self.y * rhs.1
    }
}
impl Mul for Direction {
    type Output = isize;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            let self_mag = *self * *self;
            let other_mag = *other * *other;
            if self.y.signum() * (self.y * self.y * other_mag)
                < other.y.signum() * (other.y * other.y * self_mag)
            {
                if self.x.is_negative() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if other.x.is_negative() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

impl Direction {
    #[must_use]
    pub const fn new(x: isize, y: isize) -> Self {
        let gcd = pos_gcd(x.abs(), y.abs());
        if gcd == 0 {
            Self { x: 0, y: 0 }
        } else {
            Self {
                x: x / gcd,
                y: y / gcd,
            }
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}
const fn pos_gcd(a: isize, b: isize) -> isize {
    if a < 0 {
        pos_gcd(-a, b)
    } else if b < 0 {
        pos_gcd(a, -b)
    } else if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a & 1 == 0 && b & 1 == 0 {
        2 * pos_gcd(a >> 1, b >> 1)
    } else if b & 1 == 0 {
        pos_gcd(a, b >> 1)
    } else if a & 1 == 1 && b & 1 == 1 && a <= b {
        pos_gcd(a, b - a)
    } else {
        pos_gcd(b, a)
    }
}

#[cfg(test)]
mod test {
    use crate::direction::{pos_gcd, Direction};

    #[test]
    fn gcd1() {
        let ans = pos_gcd(-5, 15);
        assert_eq!(ans, 5);
    }

    #[test]
    fn direction() {
        let ans = Direction::new(-4, -4);
        assert_eq!(ans, Direction { x: -1, y: -1 })
    }
}
