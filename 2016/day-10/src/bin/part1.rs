use std::collections::HashMap;

#[derive(Debug)]
enum Line {
    Value { value: u8, bot: u8 },
    Comparison { bot: u8, low: u8, high: u8 },
}

fn main() {
    let input = include_str!("../../input")
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();
            let values = split
                .iter()
                .filter_map(|x| {
                    let ret = x.parse::<u8>();
                    match ret {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<_>>();
            match split[0] {
                "value" => Line::Value {
                    value: values[0],
                    bot: values[1],
                },
                "bot" => Line::Comparison {
                    bot: values[0],
                    low: values[1],
                    high: values[2],
                },
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();
    let mut bots = Bots::from(&input);

    loop {
        let comps = bots.iteration();
        println!("{comps:?}");
        if let Some(x) = comps.get(&(17,61)) {
            println!("{x}");
            break;
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Bot {
    memory: [Option<u8>; 2],
    low: Option<u8>,
    high: Option<u8>,
}

impl Bot {
    fn from(memory: [Option<u8>; 2]) -> Self {
        Bot {
            memory,
            low: None,
            high: None,
        }
    }

    fn addval(&mut self, value: u8) {
        if self.memory[0].is_none() {
            self.memory[0] = Some(value);
        } else if self.memory[1].is_none() {
            if self.memory[0].unwrap() > value {
                self.memory[1] = self.memory[0];
                self.memory[0] = Some(value);
            } else {
                self.memory[1] = Some(value);
            }
        } else {
            unreachable!("Three values were added");
        }
    }

    fn considerable(&self) -> bool {
        self.memory.into_iter().all(|x| x.is_some())
    }

    fn wipe(&mut self) {
        self.memory[0] = None;
        self.memory[1] = None;
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            memory: Default::default(),
            low: Default::default(),
            high: Default::default(),
        }
    }
}

struct Bots {
    bots: HashMap<u8, Bot>,
}

impl Bots {
    fn new() -> Self {
        Bots {
            bots: HashMap::new(),
        }
    }

    fn from(lines: &[Line]) -> Self {
        let mut ret = Bots::new();
        for line in lines {
            match line {
                Line::Value { value, bot } => {
                    if let Some(x) = ret.bots.get_mut(bot) {
                        x.addval(*value);
                    } else {
                        ret.bots.insert(*bot, Bot::from([Some(*value), None]));
                    }
                }
                Line::Comparison { bot, low, high } => {
                    if let Some(x) = ret.bots.get_mut(bot) {
                        x.low = Some(*low);
                        x.high = Some(*high);
                    } else {
                        ret.bots.insert(
                            *bot,
                            Bot {
                                memory: [None; 2],
                                low: Some(*low),
                                high: Some(*high),
                            },
                        );
                    }
                }
            }
        }
        ret
    }
    fn iteration(&mut self) ->HashMap<(u8,u8), u8> {
        let consider = self
            .bots
            .iter()
            .filter(|(_, v)| v.considerable())
            .map(|(k, v)| (*k, (*v).clone()))
            .collect::<Vec<_>>();
        let ret = consider
            .iter()
            .map(|(k, v)| ((v.memory[0].unwrap(), v.memory[1].unwrap()), *k))
            .collect::<HashMap<_, _>>();
        for (curr, bot) in consider {
            self.bots
                .get_mut(&bot.low.unwrap())
                .unwrap()
                .addval(bot.memory[0].unwrap());
            self.bots
                .get_mut(&bot.high.unwrap())
                .unwrap()
                .addval(bot.memory[1].unwrap());

            self.bots.get_mut(&curr).unwrap().wipe();
        }
        ret
    }
}
