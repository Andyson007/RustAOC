use bytemuck::ByteEq;
use bytemuck::{ByteHash, NoUninit};
use fx_hash::FxHashMap;
use fx_hash::FxHashMapExt;
use std::{
    collections::BinaryHeap,
    mem::{self, MaybeUninit},
};

use nom::{
    bytes::complete::tag,
    character::{self, complete::space0},
    multi, AsChar, IResult,
};

fn main() {
    let files = Files::<33, 30>::parse(include_str!("../../input"))
        .unwrap()
        .1;
    let ans = solve(files);
    println!("{ans:?}");
}

fn solve<const X: usize, const Y: usize>(files: Files<X, Y>) -> Option<usize> {
    let mut binheap = BinaryHeap::new();

    let countable = Countable::new(files, Files::heuristic);
    binheap.push(countable);

    let mut visited = FxHashMap::new();
    visited.insert(*binheap.peek().unwrap().inner(), 0);

    let mut max = 0;
    while let Some(curr) = binheap.pop() {
        if curr.offset > max {
            max = curr.offset;
            println!("{max}");
        }
        if curr.offset == 12 {
            println!();
            println!("{}", binheap.len());
            println!("{}", visited.len());
            return None;
        }
        if curr.state.is_solved() {
            return Some(curr.offset);
        }
        visited.insert(*curr.inner(), 0);
        let mut count = 0;
        for variation in curr.state.get_next() {
            count += 1;

            if let Some(x) = visited.get_mut(&variation) {
                *x -= 1;
                if *x == 0 {
                    visited.remove(&variation);
                }
            } else {
                binheap.push(curr.next_with_inner(variation));
            }
        }
        if count == 0 {
            visited.remove(curr.inner());
        } else {
            *visited.get_mut(curr.inner()).unwrap() += count;
        }
    }
    None
}

fn pretty_print<const X: usize, const Y: usize>(data: &Files<X, Y>) {
    println!("{:?}", data.goal);
    for i in 0..Y {
        for j in 0..X {
            let data = data.files[i][j];
            print!("{}/{}\t", data.used, data.size);
        }
        println!();
    }
}

#[repr(C)]
#[derive(Debug, Clone, ByteEq, ByteHash, Copy)]
struct Files<const X: usize, const Y: usize> {
    files: [[FileData; X]; Y],
    goal: (usize, usize),
}

unsafe impl<const X: usize, const Y: usize> NoUninit for Files<X, Y> {}

impl<const X: usize, const Y: usize> Files<X, Y> {
    pub fn get_next(&self) -> impl Iterator<Item = Files<X, Y>> + use<'_, X, Y> {
        let mut ret = Vec::new();
        for ((x, y), data) in self
            .files
            .iter()
            .enumerate()
            .flat_map(|(j, x)| x.iter().enumerate().map(move |(i, x)| ((i, j), x)))
        {
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let coords: (usize, usize) = {
                    let (Ok(x), Ok(y)) = (
                        (x as isize + dir.0).try_into(),
                        (y as isize + dir.1).try_into(),
                    ) else {
                        continue;
                    };
                    if x >= X || y >= Y {
                        continue;
                    }
                    (x, y)
                };
                let other = self.files[coords.1][coords.0];
                if other.avail() >= data.used {
                    let mut cloned = *self;
                    cloned.move_data((x, y), coords);
                    ret.push(cloned);
                }
            }
        }
        ret.into_iter()
    }

    fn move_data(&mut self, from: (usize, usize), to: (usize, usize)) {
        if from == self.goal {
            self.goal = to;
        }
        let to_move = self.files[from.1][from.0].take();
        self.files[to.1][to.0].add(to_move);
    }

    pub fn heuristic(&self) -> usize {
        self.goal.0 + self.goal.1 + 1
    }

    pub fn is_solved(&self) -> bool {
        self.goal == (0, 0)
    }

    pub fn parse(data: &str) -> IResult<&str, Self> {
        let (data, _) = tag("root@ebhq-gridcenter# df -h\nFilesystem")(data)?;
        let (data, _) = space0(data)?;
        let (data, _) = tag("Size  Used  Avail  Use%\n")(data)?;
        let (i, data) = parse_lines(data)?;
        Ok((
            i,
            Self {
                goal: data
                    .iter()
                    .filter(|x| x.0 .1 == 0)
                    .max_by_key(|x| x.0 .0)
                    .unwrap()
                    .0,
                files: into_arr(data),
            },
        ))
    }
}

fn into_arr<const X: usize, const Y: usize>(
    data: Vec<((usize, usize), FileData)>,
) -> [[FileData; X]; Y] {
    assert_eq!(data.len(), X * Y);
    let mut ret = [[MaybeUninit::uninit(); X]; Y];
    for ((x, y), data) in data {
        ret[y][x] = MaybeUninit::new(data);
    }
    ret.map(|x| x.map(|x| unsafe { x.assume_init() }))
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct FileData {
    size: usize,
    used: usize,
}

impl FileData {
    pub fn avail(&self) -> usize {
        self.size - self.used
    }

    pub fn take(&mut self) -> usize {
        mem::replace(&mut self.used, 0)
    }

    pub fn add(&mut self, amount: usize) {
        self.used += amount;
        debug_assert!(self.used <= self.size, "{self:?}");
    }
}

type PreHashmap = ((usize, usize), FileData);

#[derive(Debug)]
struct Countable<T, H>
where
    H: FnMut(&T) -> usize,
{
    offset: usize,
    state: T,
    heuristic: H,
}

impl<T: Clone, H: Clone> Clone for Countable<T, H>
where
    H: FnMut(&T) -> usize,
{
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            state: self.state.clone(),
            heuristic: self.heuristic.clone(),
        }
    }
}

impl<T, H> Countable<T, H>
where
    H: Fn(&T) -> usize,
{
    pub fn new(state: T, heuristic: H) -> Self {
        Self {
            offset: 0,
            state,
            heuristic,
        }
    }

    fn weighted(&self) -> usize {
        self.offset + (self.heuristic)(&self.state)
    }

    pub fn inner(&self) -> &T {
        &self.state
    }
}

impl<T, H> Countable<T, H>
where
    H: Fn(&T) -> usize + Copy,
{
    pub fn next_with_inner(&self, inner: T) -> Self {
        Self {
            offset: self.offset + 1,
            state: inner,
            heuristic: self.heuristic,
        }
    }
}

impl<T, H> PartialOrd for Countable<T, H>
where
    H: Fn(&T) -> usize,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, H> Ord for Countable<T, H>
where
    H: Fn(&T) -> usize,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weighted().cmp(&self.weighted())
    }
}

impl<T, H> PartialEq for Countable<T, H>
where
    H: Fn(&T) -> usize,
{
    fn eq(&self, _other: &Self) -> bool {
        // FIXME: I don't think this method should be called anytime actually so :shrug:
        todo!()
    }
}

impl<T, H> Eq for Countable<T, H> where H: Fn(&T) -> usize {}

// Parsing function
fn parse_lines(i: &str) -> IResult<&str, Vec<PreHashmap>> {
    multi::separated_list0(nom::character::complete::char('\n'), parse_line)(i)
}

fn parse_line(i: &str) -> IResult<&str, PreHashmap> {
    let (i, _) = tag("/dev/grid/node-x")(i)?;
    let (i, x) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let x = x.parse::<usize>().unwrap();
    let (i, _) = tag("-y")(i)?;
    let (i, y) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let y = y.parse::<usize>().unwrap();
    let (i, _) = space0(i)?;
    let (i, size) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let size = size.parse::<usize>().unwrap();
    let (i, _) = character::complete::char('T')(i)?;
    let (i, _) = space0(i)?;
    let (i, used) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let used = used.parse::<usize>().unwrap();
    let (i, _) = character::complete::char('T')(i)?;
    let (i, _) = space0(i)?;
    let (i, _) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let (i, _) = character::complete::char('T')(i)?;
    let (i, _) = space0(i)?;
    let (i, _) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let (i, _) = character::complete::char('%')(i)?;
    Ok((i, ((x, y), FileData { size, used })))
}

#[cfg(test)]
mod test {
    use crate::{solve, Files};

    #[test]
    fn example() {
        let files = Files::<3, 3>::parse(include_str!("../../example"))
            .unwrap()
            .1;
        let ans = solve(files);
        assert_eq!(ans, Some(7));
    }
}
