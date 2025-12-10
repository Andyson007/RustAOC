struct Choices<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    choices: Vec<T>,
    iter: I,
    indicies: (usize, usize),
}

impl<T, I> Iterator for Choices<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.indicies.0 == 0 {
            self.choices.push(self.iter.next()?);
        }
        let ret = (
            self.choices[self.indicies.0].clone(),
            self.choices[self.indicies.1].clone(),
        );
        if self.indicies.0 == self.indicies.1 {
            self.indicies.0 = 0;
            self.indicies.1 += 1;
        } else {
            self.indicies.0 += 1;
        }
        Some(ret)
    }
}

trait IteratorExt: Iterator {
    fn choices(self) -> Choices<Self::Item, Self>
    where
        Self::Item: Clone,
        Self: Sized;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn choices(self) -> Choices<Self::Item, Self>
    where
        Self::Item: Clone,
        Self: Sized,
    {
        Choices {
            choices: Vec::new(),
            iter: self,
            indicies: (0, 0),
        }
    }
}
