fn main() {
    let mut fft = FFT::parse(include_str!("../../input").trim(), &[0, 1, 0, -1]);
    for _ in 0..100 {
        fft = fft.apply();
    }
    println!(
        "{}",
        fft.values()[0..8]
            .iter()
            .map(|x| char::from_digit(*x as u32, 10).unwrap())
            .collect::<String>()
    )
}

use std::fmt::Display;

pub struct FFT {
    values: Box<[i8]>,
    pattern: Box<[i8]>,
}

impl FFT {
    pub fn new(values: &[i8], pattern: &[i8]) -> Self {
        Self {
            values: Box::from(values),
            pattern: Box::from(pattern),
        }
    }

    pub fn parse(values: &str, pattern: &[i8]) -> Self {
        Self::new(
            values
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i8)
                .collect::<Vec<_>>()
                .as_slice(),
            pattern,
        )
    }

    pub fn values(&self) -> &[i8] {
        &self.values
    }

    pub fn apply(&self) -> Self {
        let mut ret = Vec::with_capacity(self.values.len());
        for i in 1..=self.values.len() {
            let pattern = self
                .pattern
                .iter()
                .flat_map(|value| std::iter::repeat(value).take(i))
                .cycle()
                .skip(1);

            ret.push(
                (self
                    .values
                    .iter()
                    .zip(pattern)
                    .map(|(elem, pattern_value)| *elem as i64 * *pattern_value as i64)
                    .sum::<i64>()
                    .abs()
                    % 10) as i8,
            );
        }
        Self::new(ret.as_slice(), &self.pattern)
    }
}

impl Display for FFT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.values
                .iter()
                .map(|x| char::from_digit(*x as u32, 10).unwrap())
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod part1_examples {
    use crate::FFT;

    #[test]
    fn example1() {
        let mut fft = FFT::parse("12345678", &[0, 1, 0, -1]);
        fft = fft.apply();
        assert_eq!(*fft.values, *val_to_slice("48226158"));
        fft = fft.apply();
        assert_eq!(*fft.values, *val_to_slice("34040438"));
        fft = fft.apply();
        assert_eq!(*fft.values, *val_to_slice("03415518"));
        fft = fft.apply();
        assert_eq!(*fft.values, *val_to_slice("01029498"));
    }

    #[test]
    fn example2() {
        let mut fft = FFT::parse("80871224585914546619083218645595", &[0, 1, 0, -1]);
        for _ in 0..100 {
            fft = fft.apply();
        }
        assert_eq!(fft.values[0..8], *val_to_slice("24176176"));
    }

    fn val_to_slice(values: &str) -> Box<[i8]> {
        Box::from_iter(values.chars().map(|x| x as i8 ^ 48))
    }
}
