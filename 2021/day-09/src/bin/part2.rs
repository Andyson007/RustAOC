use std::collections::HashSet;

use top::{Top, TopError};

fn main() -> Result<(), TopError> {
    let input = include_str!("../../input")
        .lines()
        .map(|x| x.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut topthree: Top<usize, 3> = Top::new();
    for x in 0..input.len() as i32 {
        'inner: for y in 0..input[x as usize].len() as i32 {
            let curr = input[x as usize][y as usize];
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let newpos = ((x + dir.0), (y + dir.1));
                if newpos.0 >= 0
                    && newpos.0 < input.len() as i32
                    && newpos.1 >= 0
                    && newpos.1 < input[x as usize].len() as i32
                {
                    let newpos = (newpos.0 as usize, newpos.1 as usize);
                    if input[newpos.0][newpos.1] <= curr {
                        continue 'inner;
                    }
                }
            }
            let var = &basinsize(&input, (x as usize, y as usize));
            println!("{var}");
            topthree.add(var)?;
            println!("{:?}", topthree.values)
        }
    }
    println!("{}", topthree.eval()?);
    Ok(())
}

fn basinsize(input: &[Vec<u8>], pos: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![pos];
    let mut next = Some(pos);
    while let Some(curr) = next {
        queue.swap_remove(0);
        visited.insert(curr);
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let newpos = ((curr.0 as i32 + dir.0), (curr.1 as i32 + dir.1));
            if newpos.0 >= 0
                && newpos.0 < input.len() as i32
                && newpos.1 >= 0
                && newpos.1 < input[curr.0].len() as i32
            {
                let newpos = (newpos.0 as usize, newpos.1 as usize);
                if input[newpos.0][newpos.1] != 48 ^ 9 && !visited.contains(&newpos) {
                    queue.push(newpos);
                }
            }
        }
        next = queue.first().copied();
    }
    visited.len()
}

pub mod top {
    use std::{cmp::{max, Ordering}, ops::Mul};

    #[derive(Debug, PartialEq)]
    pub enum TopError {
        NullEval,
        NoLength,
    }

    pub struct Top<T, const N: usize> {
        pub values: [Option<T>; N],
    }

    impl<T, const N: usize> Top<T, N>
    where
        T: Copy + Clone + Mul<T, Output = T> + Ord,
    {
        pub fn from(arr: &[T]) -> Result<Self, TopError> {
            let mut ret = Self::new();
            for item in arr {
                ret.add(item)?;
            }
            Ok(ret)
        }
        pub fn new() -> Self {
            Self { values: [None; N] }
        }

        pub fn add(&mut self, value: &T) -> Result<(), TopError> {
            if N == 0 {
                return Err(TopError::NoLength);
            }
            let lowest = self
                .values
                .iter_mut()
                .min_by(|a, b| {
                    if let (Some(a), Some(b)) = (a, b) {
                        a.cmp(b)
                    } else if a.is_none() && b.is_none() {
                        Ordering::Equal
                    } else if a.is_none() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap();
            if let Some(x) = lowest {
                *x = max(*x, *value);
            } else {
                *lowest = Some(*value)
            }
            Ok(())
        }

        pub fn eval(&self) -> Result<T, TopError> {
            self.values
                .iter()
                .map(|x| match x {
                    Some(x) => Ok(*x),
                    None => Err(TopError::NullEval),
                })
                .reduce(|a, b| Ok(a? * b?))
                .unwrap()
        }
    }

    impl<T, const N: usize> Default for Top<T, N>
    where
        T: Copy + Clone + Mul<T, Output = T> + Ord,
    {
        fn default() -> Self {
            self::Top::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::top::{self, Top, TopError};

    #[test]
    fn no_entries() {
        let top: Top<i32, 3> = top::Top::new();
        assert_eq!(top.eval(), Err(TopError::NullEval));
    }

    #[test]
    fn one_entry() -> Result<(), TopError> {
        let mut top: Top<i32, 3> = top::Top::new();
        top.add(&5)?;
        assert_eq!(top.eval(), Err(TopError::NullEval));
        Ok(())
    }

    #[test]
    fn almost_max_entries() -> Result<(), TopError> {
        let mut top: Top<i32, 3> = top::Top::new();
        top.add(&1)?;
        top.add(&1)?;
        assert_eq!(top.eval(), Err(TopError::NullEval));
        Ok(())
    }

    #[test]
    fn max_entries() -> Result<(), TopError> {
        let mut top: Top<i32, 3> = top::Top::new();
        top.add(&1)?;
        top.add(&1)?;
        top.add(&1)?;
        assert_eq!(top.eval(), Ok(1));
        Ok(())
    }

    #[test]
    fn different_entries() -> Result<(), TopError> {
        let mut top: Top<i32, 3> = top::Top::new();
        top.add(&1)?;
        top.add(&2)?;
        top.add(&3)?;
        assert_eq!(top.eval(), Ok(6));
        Ok(())
    }

    #[test]
    fn replacing_entries() -> Result<(), TopError> {
        const ENTRIES: usize = 3;
        let mut top: Top<i32, ENTRIES> = top::Top::new();
        let max = 16;
        for i in 0..max {
            top.add(&i)?;
        }
        assert_eq!(
            top.eval(),
            Ok((0..max).rev().take(ENTRIES).reduce(|a, b| a * b).unwrap())
        );
        Ok(())
    }

    #[test]
    fn change_entry() -> Result<(), TopError> {
        let arr = [99, 103, 100];
        let mut top: Top<i32, 3> = top::Top::from(&arr)?;
        top.add(&54)?;
        assert_eq!(top.eval()?, arr.iter().copied().reduce(|a,b|a*b).unwrap());
        Ok(())
    }
}
