pub mod planet;
pub mod system;
pub mod vector;

/// Least Common multiple
/// ```
/// # use day_12::lcm;
/// assert_eq!(lcm(12, 4), 12);
/// assert_eq!(lcm(2, 3), 6);
/// ```
pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        gcd(b, a)
    } else if a == 0 {
        return b;
    } else if a & 1 == 0 && b & 1 == 0 {
        return 2 * gcd(a / 2, b / 2);
    } else if a & 1 == 1 && b & 1 == 1 {
        gcd(a, b - a)
    } else {
        gcd(a >> a.trailing_zeros(), b >> b.trailing_zeros())
    }
}

#[cfg(test)]
mod test {
    use crate::gcd;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(2, 3), 1);
    }

    #[test]
    fn ccd_test() {
        assert_eq!(gcd(2, 3), 1);
    }
}
