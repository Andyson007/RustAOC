//Notes on this one is that the input file is so small that
//i just converted them manually

// input file is
// at the bottom of the file

use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
    ops::{Add, Sub},
};

struct VisitLookup {
    states: Vec<State>,
}

impl VisitLookup {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    pub fn contains(&self, state: &State) -> bool {
        self.states.contains(state)
    }

    pub fn insert(&mut self, state: State) -> bool {
        if self.contains(&state) {
            true
        } else {
            self.states.push(state);
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pair {
    gen: usize,
    chip: usize,
}

impl Pair {
    pub fn new(gen: usize, chip: usize) -> Self {
        Self { gen, chip }
    }
}

impl Add<(usize, usize)> for Pair {
    type Output = Self;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self {
            gen: self.gen + rhs.0,
            chip: self.chip + rhs.1,
        }
    }
}

impl Sub<(usize, usize)> for Pair {
    type Output = Self;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        Self {
            gen: self.gen - rhs.0,
            chip: self.chip - rhs.1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    state: HashMap<Pair, usize>,
    floor: usize,
}

impl State {
    pub fn heuristic(&self) -> usize {
        self.state.iter().map(|x| 8 - x.0.gen - x.0.chip).sum()
    }

    pub fn is_solved(&self) -> bool {
        match self.state.len() {
            0 => true,
            1 => self.state.contains_key(&Pair::new(3, 3)),
            _ => false,
        }
    }

    fn is_valid(&self) -> bool {
        self.state
            .iter()
            .map(|x| x.0)
            .filter(|x| x.gen != x.chip)
            .all(|pair| !self.state.iter().map(|x| x.0).any(|y| y.gen == pair.chip))
    }

    pub fn get_variations(&self) -> impl Iterator<Item = State> + use<'_> {
        self.get_double_move_variations()
            .chain(self.get_single_move_variations())
            .filter(Self::is_valid)
    }

    fn get_single_move_variations(&self) -> impl Iterator<Item = State> + use<'_> {
        self.state
            .iter()
            .filter(|(Pair { gen, .. }, _count)| *gen == self.floor)
            .flat_map(|(elem, _count)| {
                let mut cloned = self.state.clone();
                match cloned.entry(*elem) {
                    Entry::Occupied(mut x) => {
                        *x.get_mut() -= 1;
                        if *x.get() == 0 {
                            x.remove();
                        }
                        let mut ret = Vec::with_capacity(2);
                        if self.floor != 3 {
                            let mut cloned = cloned.clone();
                            *cloned.entry(*elem + (1, 0)).or_insert(0) += 1;
                            ret.push(State {
                                state: cloned,
                                floor: self.floor + 1,
                            });
                        }
                        if self.floor != 0 {
                            let mut cloned = cloned.clone();
                            *cloned.entry(*elem - (1, 0)).or_insert(0) += 1;
                            ret.push(State {
                                state: cloned,
                                floor: self.floor - 1,
                            });
                        }
                        ret
                    }
                    Entry::Vacant(_) => unreachable!(),
                }
            })
            .chain(
                self.state
                    .iter()
                    .filter(|(Pair { chip, .. }, _count)| *chip == self.floor)
                    .flat_map(|(elem, _count)| {
                        let mut cloned = self.state.clone();
                        match cloned.entry(*elem) {
                            Entry::Occupied(mut x) => {
                                *x.get_mut() -= 1;
                                if *x.get() == 0 {
                                    x.remove();
                                }
                                let mut ret = Vec::with_capacity(2);
                                if self.floor != 3 {
                                    let mut cloned = cloned.clone();
                                    *cloned.entry(*elem + (0, 1)).or_insert(0) += 1;
                                    ret.push(State {
                                        state: cloned,
                                        floor: self.floor + 1,
                                    });
                                }
                                if self.floor != 0 {
                                    let mut cloned = cloned.clone();
                                    *cloned.entry(*elem - (0, 1)).or_insert(0) += 1;
                                    ret.push(State {
                                        state: cloned,
                                        floor: self.floor - 1,
                                    });
                                }
                                ret
                            }
                            Entry::Vacant(_) => unreachable!(),
                        }
                    }),
            )
    }

    pub fn get_double_move_variations(&self) -> impl Iterator<Item = State> + use<'_> {
        self.state
            .iter()
            .filter(|(Pair { gen, .. }, _count)| *gen == self.floor)
            .flat_map(|x| {
                self.state
                    .iter()
                    .filter(|(Pair { chip, .. }, _count)| *chip == self.floor)
                    .map(move |y| (x, y))
            })
            .flat_map(|((gen_elem, _gen_count), (chip_elem, _chip_count))| {
                if gen_elem == chip_elem {
                    let mut cloned = self.state.clone();
                    match cloned.entry(*gen_elem) {
                        Entry::Occupied(mut x) => {
                            *x.get_mut() -= 1;
                            if *x.get() == 0 {
                                x.remove();
                            }
                            let mut ret = Vec::with_capacity(2);
                            if self.floor != 3 {
                                let mut cloned = cloned.clone();
                                *cloned.entry(*gen_elem + (1, 1)).or_insert(0) += 1;
                                ret.push(State {
                                    state: cloned,
                                    floor: self.floor + 1,
                                });
                            }
                            if self.floor != 0 {
                                let mut cloned = cloned.clone();
                                *cloned.entry(*gen_elem - (1, 1)).or_insert(0) += 1;
                                ret.push(State {
                                    state: cloned,
                                    floor: self.floor - 1,
                                });
                            }
                            ret
                        }
                        Entry::Vacant(_) => unreachable!(),
                    }
                } else {
                    let mut cloned = self.state.clone();
                    for elem in [gen_elem, chip_elem] {
                        let Entry::Occupied(mut x) = cloned.entry(*elem) else {
                            unreachable!()
                        };
                        *x.get_mut() -= 1;
                        if *x.get() == 0 {
                            x.remove();
                        }
                    }
                    let mut ret = Vec::with_capacity(2);
                    if self.floor != 3 {
                        let mut cloned = cloned.clone();
                        *cloned.entry(*gen_elem + (1, 0)).or_insert(0) += 1;
                        *cloned.entry(*chip_elem + (0, 1)).or_insert(0) += 1;
                        ret.push(State {
                            state: cloned,
                            floor: self.floor + 1,
                        });
                    }
                    if self.floor != 0 {
                        let mut cloned = cloned.clone();
                        *cloned.entry(*gen_elem - (1, 0)).or_insert(0) += 1;
                        *cloned.entry(*chip_elem - (0, 1)).or_insert(0) += 1;
                        ret.push(State {
                            state: cloned,
                            floor: self.floor - 1,
                        });
                    }
                    ret
                }
            })
            .chain(
                self.state
                    .iter()
                    .filter(|(Pair { gen, .. }, _)| *gen == self.floor)
                    .choices()
                    .filter(
                        |((a, count), (b, _))| {
                            if a == b {
                                **count >= 2
                            } else {
                                true
                            }
                        },
                    )
                    .flat_map(|((a, _), (b, _))| {
                        let mut cloned = self.state.clone();
                        for elem in [a, b] {
                            let Entry::Occupied(mut x) = cloned.entry(*elem) else {
                                unreachable!()
                            };
                            *x.get_mut() -= 1;
                            if *x.get() == 0 {
                                x.remove();
                            }
                        }

                        let mut ret = Vec::with_capacity(2);
                        if self.floor != 3 {
                            let mut cloned = cloned.clone();
                            *cloned.entry(*a + (1, 0)).or_insert(0) += 1;
                            *cloned.entry(*b + (1, 0)).or_insert(0) += 1;
                            ret.push(State {
                                state: cloned,
                                floor: self.floor + 1,
                            });
                        }
                        if self.floor != 0 {
                            let mut cloned = cloned.clone();
                            *cloned.entry(*a - (1, 0)).or_insert(0) += 1;
                            *cloned.entry(*b - (1, 0)).or_insert(0) += 1;
                            ret.push(State {
                                state: cloned,
                                floor: self.floor - 1,
                            });
                        }
                        ret
                    }),
            )
            .chain(
                self.state
                    .iter()
                    .filter(|(Pair { chip, .. }, _)| *chip == self.floor)
                    .choices()
                    .filter(
                        |((a, count), (b, _))| {
                            if a == b {
                                **count >= 2
                            } else {
                                true
                            }
                        },
                    )
                    .flat_map(|((a, _), (b, _))| {
                        let mut cloned = self.state.clone();
                        for elem in [a, b] {
                            let Entry::Occupied(mut x) = cloned.entry(*elem) else {
                                unreachable!()
                            };
                            *x.get_mut() -= 1;
                            if *x.get() == 0 {
                                x.remove();
                            }
                        }

                        let mut ret = Vec::with_capacity(2);
                        if self.floor != 3 {
                            let mut cloned = cloned.clone();
                            *cloned.entry(*a + (0, 1)).or_insert(0) += 1;
                            *cloned.entry(*b + (0, 1)).or_insert(0) += 1;
                            ret.push(State {
                                state: cloned,
                                floor: self.floor + 1,
                            });
                        }
                        if self.floor != 0 {
                            let mut cloned = cloned.clone();
                            *cloned.entry(*a - (0, 1)).or_insert(0) += 1;
                            *cloned.entry(*b - (0, 1)).or_insert(0) += 1;
                            ret.push(State {
                                state: cloned,
                                floor: self.floor - 1,
                            });
                        }
                        ret
                    }),
            )
    }
}

#[derive(Debug)]
struct Countable<T, H>
where
    H: FnMut(&T) -> usize,
{
    offset: usize,
    state: T,
    heuristic: H,
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

fn main() {
    let input = input();
    let mut binheap = BinaryHeap::new();

    binheap.push(Countable::new(State::from(&input), State::heuristic));

    let mut visited = VisitLookup::new();
    visited.insert(binheap.peek().unwrap().inner().clone());

    let mut max = 0;

    while let Some(curr) = binheap.pop() {
        // println!("{:?}", curr.state);
        if curr.offset > max {
            max = curr.offset;
            // println!("max: {max}");
        }
        if curr.state.is_solved() {
            println!("Solved: {}", curr.offset);
            break;
        }

        for variation in curr.state.get_variations() {
            if !visited.contains(&variation) {
                visited.insert(variation.clone());
                binheap.push(curr.next_with_inner(variation));
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ExplicitState {
    state: [HashSet<Thing>; 4],
    elevator: usize,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Thing {
    r#type: Type,
    element: Element,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Element {
    Hydrogen,
    Polonium,
    Promethium,
    Thulium,
    Ruthenium,
    Cobalt,
    Elerium,
    Dilithium,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Type {
    Generator,
    Microchip,
}

// The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip, a promethium generator,
// a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator, and a cobalt-compatible microchip.
// The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
// The third floor contains nothing relevant.
// The fourth floor contains nothing relevant.

fn input() -> ExplicitState {
    let arr = [
        HashSet::from([
            Thing {
                r#type: Type::Generator,
                element: Element::Polonium,
            },
            Thing {
                r#type: Type::Generator,
                element: Element::Thulium,
            },
            Thing {
                r#type: Type::Microchip,
                element: Element::Thulium,
            },
            Thing {
                r#type: Type::Generator,
                element: Element::Promethium,
            },
            Thing {
                r#type: Type::Generator,
                element: Element::Ruthenium,
            },
            Thing {
                r#type: Type::Microchip,
                element: Element::Ruthenium,
            },
            Thing {
                r#type: Type::Microchip,
                element: Element::Cobalt,
            },
            Thing {
                r#type: Type::Generator,
                element: Element::Cobalt,
            },
            // Thing {
            //     r#type: Type::Microchip,
            //     element: Element::Elerium,
            // },
            // Thing {
            //     r#type: Type::Generator,
            //     element: Element::Elerium,
            // },
            // Thing {
            //     r#type: Type::Microchip,
            //     element: Element::Dilithium,
            // },
            // Thing {
            //     r#type: Type::Generator,
            //     element: Element::Dilithium,
            // },
        ]),
        // The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
        HashSet::from([
            Thing {
                r#type: Type::Microchip,
                element: Element::Polonium,
            },
            Thing {
                r#type: Type::Microchip,
                element: Element::Promethium,
            },
        ]),
        HashSet::new(),
        HashSet::new(),
    ];
    ExplicitState {
        elevator: 0,
        state: arr,
    }
}

fn example() -> ExplicitState {
    let arr = [
        HashSet::from([]),
        HashSet::from([]),
        HashSet::from([
            Thing {
                r#type: Type::Generator,
                element: Element::Dilithium,
            },
            Thing {
                r#type: Type::Generator,
                element: Element::Promethium,
            },
            Thing {
                r#type: Type::Microchip,
                element: Element::Promethium,
            },
        ]),
        HashSet::from([Thing {
            r#type: Type::Microchip,
            element: Element::Dilithium,
        }]),
    ];
    ExplicitState {
        elevator: 2,
        state: arr,
    }
}

impl From<&ExplicitState> for State {
    fn from(state: &ExplicitState) -> Self {
        let mut ret = HashMap::new();
        for (floor, element) in state
            .state
            .iter()
            .enumerate()
            .flat_map(|(i, x)| x.iter().map(move |x| (i, x)))
            .filter(|(_, x)| x.r#type == Type::Generator)
            .map(|(i, x)| (i, x.element))
        {
            let chip_elem = Thing {
                r#type: Type::Microchip,
                element,
            };
            let chip_floor = state
                .state
                .iter()
                .enumerate()
                .find(|(_, x)| x.contains(&chip_elem))
                .unwrap()
                .0;
            *ret.entry(Pair::new(floor, chip_floor)).or_insert(0) += 1;
        }
        Self {
            state: ret,
            floor: state.elevator,
        }
    }
}

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
