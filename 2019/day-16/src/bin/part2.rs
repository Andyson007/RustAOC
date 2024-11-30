use std::fmt::Display;

fn main() {
    let (mut fft, offset) = FFT::parse(include_str!("../../input").trim());
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

pub struct FFT {
    values: Box<[u8]>,
}

impl FFT {
    pub fn new(values: &[u8]) -> Self {
        Self {
            values: Box::from_iter(
                std::iter::once(1).chain(
                    std::iter::repeat(values.iter().cloned())
                        .take(1)
                        .flatten(),
                ),
            ),
        }
    }

    pub fn parse(values: &str) -> (Self, usize) {
        (
            Self::new(
                values
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            values
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .take(7)
                .fold(0, |sum, curr| sum * 10 + curr as usize),
        )
    }

    pub fn values(&self) -> &[u8] {
        &self.values
    }

    pub fn apply(&self) -> Self {
        Self {
            values: Box::from_iter((1..=self.values.len()).map(|i| {
                let mut iter = self.values.chunks(i);
                iter.next();
                let mut sum = 0isize;
                while let Some(x) = iter.next() {
                    sum += x.iter().fold(0, |sum, curr| {
                        let added = sum + curr;
                        if added > 10 {
                            added - 10
                        } else {
                            added
                        }
                    }) as isize;

                    iter.next();

                    sum -= iter.next().into_iter().flatten().fold(0, |sum, curr| {
                        let added = sum + curr;
                        if added > 10 {
                            added - 10
                        } else {
                            added
                        }
                    }) as isize;
                }
                (sum.abs() % 10) as u8
            })),
        }
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
mod part2_examples {
    use crate::FFT;

    #[test]
    fn example1() {
        test_values("03036732577212944063491565474664", "84462026");
    }

    #[test]
    fn example2() {
        test_values("02935109699940807407585447034323", "78725270");
    }

    #[test]
    fn example3() {
        test_values("03081770884921959731165446850517", "53553731");
    }

    fn test_values(input: &str, output: &str) {
        let (mut fft, offset) = FFT::parse(input.trim());
        for _ in 0..100 {
            fft = fft.apply();
        }
        assert_eq!(fft.values()[offset..offset + 8], *val_to_slice(output))
    }

    fn val_to_slice(values: &str) -> Box<[u8]> {
        Box::from_iter(values.chars().map(|x| x as u8 ^ 48))
    }
}
