pub struct Permutations<T>
where
    T: Iterator,
{
    values: Vec<T::Item>,
    current_indicies: Vec<(Magnitude, usize)>,
    first: bool,
}

impl<T> Iterator for Permutations<T>
where
    T: Iterator,
    T::Item: Clone,
{
    type Item = Vec<T::Item>;
    fn next(&mut self) -> Option<Self::Item>
    where
        T::Item: Clone,
    {
        if self.values.is_empty() {
            return None;
        }

        if self.first {
            self.first = false;
            return Some(
                self.current_indicies
                    .iter()
                    .map(|(_, i)| self.values[*i].clone())
                    .collect(),
            );
        }

        let (pos, (magnitude, value)) = self
            .current_indicies
            .iter()
            .enumerate()
            .filter(|(_, (x, _))| !matches!(x, Magnitude::Zero))
            .max_by_key(|x| x.1 .1)?;

        let magnitude = *magnitude;
        let value = *value;
        let swap_with_pos = (pos as isize + magnitude.magnitude()) as usize;

        self.current_indicies.swap(pos, swap_with_pos);
        if swap_with_pos == 0 || swap_with_pos == self.current_indicies.len() - 1 {
            self.current_indicies[swap_with_pos].0 = Magnitude::Zero;
        } else {
            let next_swap_pos = (swap_with_pos as isize + magnitude.magnitude()) as usize;
            if self.current_indicies[next_swap_pos].1 > value {
                self.current_indicies[swap_with_pos].0 = Magnitude::Zero;
            }
        }

        if swap_with_pos != 0 {
            for elem in self
                .current_indicies
                .iter_mut()
                .take(swap_with_pos)
                .filter(|(_, curr_value)| *curr_value > value)
                .map(|x| &mut x.0)
            {
                *elem = Magnitude::Positive;
            }
        }

        for elem in self
            .current_indicies
            .iter_mut()
            .skip(swap_with_pos + 1)
            .filter(|(_, curr_value)| *curr_value > value)
            .map(|x| &mut x.0)
        {
            *elem = Magnitude::Negative;
        }

        Some(
            self.current_indicies
                .iter()
                .map(|(_, i)| self.values[*i].clone())
                .collect(),
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum Magnitude {
    Negative,
    Zero,
    Positive,
}

impl Magnitude {
    pub fn magnitude(self) -> isize {
        match self {
            Magnitude::Negative => -1,
            Magnitude::Zero => 0,
            Magnitude::Positive => 1,
        }
    }
}

fn permutations<T>(iter: &mut T) -> Permutations<T>
where
    T: Iterator,
{
    let values = iter.collect::<Vec<_>>();
    let len = values.len();
    Permutations {
        values,
        current_indicies: (0..)
            .take(len)
            .map(|x| {
                if x == 0 {
                    (Magnitude::Zero, 0)
                } else {
                    (Magnitude::Negative, x)
                }
            })
            .collect(),
        first: true,
    }
}
pub trait IteratorExt: Iterator {
    fn permutations(&mut self) -> Permutations<Self>
    where
        Self: Sized,
        Self::Item: Clone;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn permutations(&mut self) -> Permutations<Self> {
        permutations(self)
    }
}

#[cfg(test)]
mod test {
    use crate::IteratorExt;

    #[test]
    fn empty() {
        assert_eq!(std::iter::empty::<()>().permutations().count(), 0);
    }

    #[test]
    fn one() {
        let mut iter = std::iter::once(5).permutations();
        assert_eq!(iter.next(), Some(vec![5]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two() {
        let mut iter = (0..2).permutations();
        assert_eq!(iter.next(), Some(vec![0, 1]));
        assert_eq!(iter.next(), Some(vec![1, 0]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn three() {
        let mut iter = (1..4).permutations();
        assert_eq!(iter.next(), Some(vec![1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![1, 3, 2]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2]));
        assert_eq!(iter.next(), Some(vec![3, 2, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3, 1]));
        assert_eq!(iter.next(), Some(vec![2, 1, 3]));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn five_len() {
        let iter = (0..5).permutations();
        assert_eq!(iter.count(), 120);
    }
}
