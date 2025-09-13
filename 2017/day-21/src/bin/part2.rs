use std::{array, collections::HashMap, str::FromStr};

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

fn solve(input: &str) -> usize {
    let rules = Rules::from_str(input).unwrap();
    let pattern = Pattern::from_str(".#./..#/###").unwrap();
    let mut artwork = ArtWork::from_pattern(&pattern);

    for _ in 0..18 {
        artwork.iterate(&rules);
    }
    artwork.count_lights()
}

struct Rules {
    rules: [HashMap<usize, Pattern>; 2],
}

impl FromStr for Rules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = array::from_fn(|_| HashMap::new());
        for line in s.lines() {
            let (condition, result) = line.split_once(" => ").unwrap();
            let condition_pattern = Pattern::from_str(condition)?;
            let result_pattern = Pattern::from_str(result)?;

            rules[condition_pattern.degree() - 2]
                .insert(get_min_value(&condition_pattern), result_pattern);
        }
        Ok(Self { rules })
    }
}

struct ArtWork {
    size: usize,
    pixels: Vec<Vec<bool>>,
}

impl ArtWork {
    pub fn from_pattern(pat: &Pattern) -> Self {
        match pat {
            Pattern::Two(x) => Self {
                size: 2,
                pixels: x.iter().map(|x| x.into()).collect(),
            },
            Pattern::Three(x) => Self {
                size: 3,
                pixels: x.iter().map(|x| x.into()).collect(),
            },
            Pattern::Four(x) => Self {
                size: 4,
                pixels: x.iter().map(|x| x.into()).collect(),
            },
        }
    }
    pub fn print_artwork(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}", if self.pixels[x][y] { "██" } else { "  " })
            }
            println!()
        }
        println!()
    }

    pub fn count_lights(&self) -> usize {
        self.pixels
            .iter()
            .map(|x| x.iter().filter(|x| **x).count())
            .sum()
    }

    pub fn new(art: Vec<Vec<bool>>) -> Self {
        Self {
            size: art.len(),
            pixels: art,
        }
    }

    pub fn iterate(&mut self, rules: &Rules) {
        let new_size;
        let mut ret;
        if self.size % 2 == 0 {
            new_size = self.size * 3 / 2;
            ret = vec![vec![false; new_size]; new_size];
            for i in (0..self.size).step_by(2) {
                for j in (0..self.size).step_by(2) {
                    let pat = Pattern::Two([
                        [self.pixels[i][j], self.pixels[i][j + 1]],
                        [self.pixels[i + 1][j], self.pixels[i + 1][j + 1]],
                    ]);
                    let Pattern::Three(expansion) =
                        rules.rules[0].get(&get_min_value(&pat)).unwrap()
                    else {
                        unreachable!();
                    };
                    for x in 0..3 {
                        for y in 0..3 {
                            ret[i * 3 / 2 + x][j * 3 / 2 + y] = expansion[x][y];
                        }
                    }
                }
            }
        } else {
            assert!(self.size % 3 == 0);
            new_size = self.size * 4 / 3;
            ret = vec![vec![false; new_size]; new_size];
            for i in (0..self.size).step_by(3) {
                for j in (0..self.size).step_by(3) {
                    let pat = Pattern::Three([
                        [
                            self.pixels[i][j],
                            self.pixels[i][j + 1],
                            self.pixels[i][j + 2],
                        ],
                        [
                            self.pixels[i + 1][j],
                            self.pixels[i + 1][j + 1],
                            self.pixels[i + 1][j + 2],
                        ],
                        [
                            self.pixels[i + 2][j],
                            self.pixels[i + 2][j + 1],
                            self.pixels[i + 2][j + 2],
                        ],
                    ]);
                    let Pattern::Four(expansion) =
                        rules.rules[1].get(&get_min_value(&pat)).unwrap()
                    else {
                        unreachable!();
                    };
                    for x in 0..4 {
                        for y in 0..4 {
                            ret[i * 4 / 3 + x][j * 4 / 3 + y] = expansion[x][y];
                        }
                    }
                }
            }
        }
        *self = Self {
            size: new_size,
            pixels: ret,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Pattern {
    Two([[bool; 2]; 2]),
    Three([[bool; 3]; 3]),
    Four([[bool; 4]; 4]),
}

impl Pattern {
    pub fn degree(&self) -> usize {
        match self {
            Pattern::Two(_) => 2,
            Pattern::Three(_) => 3,
            Pattern::Four(_) => 4,
        }
    }

    pub fn split(self) -> impl Iterator<Item = Pattern> {
        match self {
            Pattern::Two(_) => vec![self].into_iter(),
            Pattern::Three(_) => vec![self].into_iter(),
            Pattern::Four([[a, b, c, d], [e, f, g, h], [i, j, k, l], [n, m, o, p]]) => vec![
                Pattern::Two([[a, b], [e, f]]),
                Pattern::Two([[c, d], [g, h]]),
                Pattern::Two([[i, j], [n, m]]),
                Pattern::Two([[k, l], [o, p]]),
            ]
            .into_iter(),
        }
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .split('/')
            .map(|line| line.chars().map(|x| x == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(match vec.len() {
            2 => {
                let flattened = vec.into_iter().map(|x| [x[0], x[1]]).collect::<Vec<_>>();
                Pattern::Two([flattened[0], flattened[1]])
            }
            3 => {
                let flattened = vec
                    .into_iter()
                    .map(|x| [x[0], x[1], x[2]])
                    .collect::<Vec<_>>();
                Pattern::Three([flattened[0], flattened[1], flattened[2]])
            }
            4 => {
                let flattened = vec
                    .into_iter()
                    .map(|x| [x[0], x[1], x[2], x[3]])
                    .collect::<Vec<_>>();
                Pattern::Four([flattened[0], flattened[1], flattened[2], flattened[3]])
            }
            _ => return Err(()),
        })
    }
}

fn count_lights(pat: &str) -> usize {
    pat.chars().filter(|x| *x == '#').count()
}

fn get_min_value(pat: &Pattern) -> usize {
    [
        pat,
        &rotate_pattern(pat),
        &rotate_pattern(&rotate_pattern(pat)),
        &rotate_pattern(&rotate_pattern(&rotate_pattern(pat))),
        &flip_pattern(pat),
        &rotate_pattern(&flip_pattern(pat)),
        &rotate_pattern(&rotate_pattern(&flip_pattern(pat))),
        &rotate_pattern(&rotate_pattern(&rotate_pattern(&flip_pattern(pat)))),
    ]
    .into_iter()
    .map(evaluate_pattern)
    .min()
    .unwrap()
}

fn evaluate_pattern(pat: &Pattern) -> usize {
    match pat {
        Pattern::Two(x) => x
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, x)| if *x { 1 << i } else { 0 })
            .sum(),
        Pattern::Three(x) => x
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, x)| if *x { 1 << i } else { 0 })
            .sum(),
        Pattern::Four(x) => x
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, x)| if *x { 1 << i } else { 0 })
            .sum(),
    }
}

fn rotate_pattern(pat: &Pattern) -> Pattern {
    match pat {
        Pattern::Two([[a, b], [c, d]]) => Pattern::Two([[*b, *d], [*a, *c]]),
        Pattern::Three([[a, b, c], [d, e, f], [h, i, j]]) => {
            Pattern::Three([[*c, *f, *j], [*b, *e, *i], [*a, *d, *h]])
        }
        Pattern::Four([[a, b, c, d], [e, f, g, h], [i, j, k, l], [n, m, o, p]]) => Pattern::Four([
            [*d, *h, *l, *p],
            [*c, *g, *k, *o],
            [*b, *f, *j, *m],
            [*a, *e, *i, *n],
        ]),
    }
}

fn flip_pattern(pat: &Pattern) -> Pattern {
    match pat {
        Pattern::Two([a, b]) => Pattern::Two([*b, *a]),
        Pattern::Three([a, b, c]) => Pattern::Three([*c, *b, *a]),
        Pattern::Four([a, b, c, d]) => Pattern::Four([*d, *c, *b, *a]),
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{evaluate_pattern, get_min_value, rotate_pattern, Pattern, Rules};

    #[test]
    fn min_value() {
        for elem in ["#./#.", "##/..", "../##", ".#/.#"] {
            let min = get_min_value(&Pattern::from_str(elem).unwrap());
            assert_eq!(min, 3);
        }
    }
}
