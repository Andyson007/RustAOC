//Notes on this one is that the input file is so small that
//i just converted them manually

// input file is
// at the bottom of the file

use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

fn main() {
    let mut binheap = BinaryHeap::new();

    binheap.clear();
    binheap.push(input());

    let mut visited = Vec::new();
    visited.push(binheap.peek().unwrap().clone());

    let mut max = 0;

    while let Some(x) = binheap.pop() {
        if x.count> max {
            max = x.count;
            println!("{max}");
        }
        if x.state.iter().skip(1).all(|x| x.is_empty()) {
            println!("{}", x.count);
            break;
        }

        let copy = State {
            count: x.count + 1,
            ..x.clone()
        };

        for elem in x.state[x.elevator].iter() {
            let mut topush = copy.clone();
            topush.state[x.elevator].remove(elem);
            if x.elevator != 0 {
                topush.elevator -= 1;
                topush.state[x.elevator - 1].insert(*elem);
                if test(&topush.state) && !visited.contains(&topush) {
                    visited.push(topush.clone());
                    binheap.push(topush.clone());
                }
            }
            let mut topush = copy.clone();
            topush.state[x.elevator].remove(elem);
            if x.elevator != 3 {
                topush.elevator += 1;
                topush.state[x.elevator + 1].insert(*elem);
                if test(&topush.state) && !visited.contains(&topush) {
                    visited.push(topush.clone());
                    binheap.push(topush.clone());
                }
            }
        }

        for (a, b) in x.state[x.elevator].iter().tuple_combinations() {
            let mut topush = copy.clone();
            topush.state[x.elevator].remove(a);
            topush.state[x.elevator].remove(b);
            if x.elevator != 0 {
                topush.elevator -= 1;
                topush.state[x.elevator - 1].insert(*a);
                topush.state[x.elevator - 1].insert(*b);
                if test(&topush.state) && !visited.contains(&topush) {
                    visited.push(topush.clone());
                    binheap.push(topush.clone());
                }
            }

            let mut topush = copy.clone();
            topush.state[x.elevator].remove(a);
            topush.state[x.elevator].remove(b);
            if x.elevator != 3 {
                topush.elevator += 1;
                topush.state[x.elevator + 1].insert(*a);
                topush.state[x.elevator + 1].insert(*b);
                if test(&topush.state) && !visited.contains(&topush) {
                    visited.push(topush.clone());
                    binheap.push(topush.clone());
                }
            }
        }
    }

    // for entry in visited {
    //     println!("{entry:#?}");
    //     println!();
    // }
}

fn test(totest: &[HashSet<Thing>; 4]) -> bool {
    totest.iter().all(|v| {
        let generators = v
            .iter()
            .filter(|x| x.r#type == Type::Generator)
            .map(|x| x.element)
            .collect::<HashSet<_>>();
        let vulnerable = v
            .iter()
            .filter(|x| x.r#type == Type::Microchip)
            .filter(|x| !generators.contains(&x.element))
            .collect::<HashSet<_>>();
        vulnerable.is_empty() || generators.is_empty()
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    count: usize,
    state: [HashSet<Thing>; 4],
    elevator: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            (other
                .state
                .iter()
                .enumerate()
                .map(|(count, value)| count * value.len())
                .sum::<usize>()
                + other.count)
                .cmp(
                    &(self
                        .state
                        .iter()
                        .enumerate()
                        .map(|(count, value)| count * value.len())
                        .sum::<usize>()
                        + self.count),
                ),
        )
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Thing {
    r#type: Type,
    element: Element,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Element {
    Polonium,
    Promethium,
    Thulium,
    Ruthenium,
    Cobalt,
    Elerium,
    Dilithium
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

fn input() -> State {
    let mut arr = [
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
            Thing {
                r#type: Type::Microchip,
                element: Element::Elerium,
            },
            Thing {
                r#type: Type::Generator,
                element: Element::Elerium,
            },
            Thing {
                r#type: Type::Microchip,
                element: Element::Dilithium,
            },
            Thing {
                r#type: Type::Generator,
                element: Element::Dilithium,
            },
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
    arr.reverse();
    State {
        elevator: 3,
        count: 0,
        state: arr,
    }
}
