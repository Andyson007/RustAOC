#[derive(Debug, Clone)]
pub struct Image<const ROWS: usize, const COLUMNS: usize> {
    layers: Vec<[[u32; COLUMNS]; ROWS]>,
}

impl<const ROWS: usize, const COLUMNS: usize> Image<ROWS, COLUMNS> {
    pub fn from(data: Vec<u32>) -> Self {
        Self {
            layers: data
                .chunks(ROWS * COLUMNS)
                .map(|x| {
                    let mut ret = [[0; COLUMNS]; ROWS];
                    for col in 0..COLUMNS {
                        for row in 0..ROWS {
                            ret[row][col] = x[row + col * ROWS];
                        }
                    }
                    ret
                })
                .collect(),
        }
    }

    pub fn part1(&self) -> usize {
        let [_, ones, twos] = self
            .layers
            .iter()
            .map(|x| {
                [
                    x.iter()
                        .map(|x| x.iter().filter(|x| **x == 0).count())
                        .sum::<usize>(),
                    x.iter()
                        .map(|x| x.iter().filter(|x| **x == 1).count())
                        .sum::<usize>(),
                    x.iter()
                        .map(|x| x.iter().filter(|x| **x == 2).count())
                        .sum::<usize>(),
                ]
            })
            .min_by_key(|x| x[0])
            .unwrap();
        ones * twos
    }

    pub fn part2(&self) -> [[Option<bool>; COLUMNS]; ROWS] {
        let mut ret = [[None; COLUMNS]; ROWS];
        for layer in &self.layers {
            for col in 0..COLUMNS {
                for row in 0..ROWS {
                    if ret[row][col].is_none() {
                        ret[row][col] = match layer[row][col] {
                            0 => Some(false),
                            1 => Some(true),
                            2 => None,
                            _ => unreachable!()
                        }
                    }
                }
            }
        }
        ret
    }
}
