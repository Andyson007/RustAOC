use std::collections::HashMap;

#[derive(Debug)]
enum Line {
    Value {
        value: u8,
        bot: Location,
    },
    Comparison {
        bot: u8,
        low: Location,
        high: Location,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Bottype {
    Output,
    Bot,
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
                    bot: Location {
                        bottype: match split[4] {
                            "bot" => Bottype::Bot,
                            "output" => Bottype::Output,
                            _ => unreachable!(),
                        },
                        id: values[1],
                    },
                },
                "bot" => Line::Comparison {
                    bot: values[0],
                    low: Location {
                        bottype: match split[5] {
                            "bot" => Bottype::Bot,
                            "output" => Bottype::Output,
                            _ => unreachable!(),
                        },
                        id: values[1],
                    },
                    high: Location {
                        bottype: match split[10] {
                            "bot" => Bottype::Bot,
                            "output" => Bottype::Output,
                            _ => unreachable!(),
                        },
                        id: values[2],
                    },
                },
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();
    let mut bots = Bots::from(&input);

    loop {
        let comps = bots.iteration();
        // println!("{comps:?}");
        if comps.is_empty() {
            break;
        }
    }
    let mut ans = 1;
    for i in 0..=2 {
        ans *= bots
            .bots
            .get(&Location {
                bottype: Bottype::Output,
                id: i,
            })
            .unwrap()
            .memory[0]
            .unwrap() as u64;
    }
    println!("{ans}");
}

#[derive(Copy, Clone, Debug, Default)]
struct Bot {
    memory: [Option<u8>; 2],
    low: Option<Location>,
    high: Option<Location>,
}

impl Bot {
    fn count(&self) -> u8 {
        self.memory.iter().filter(|x| x.is_some()).count() as u8
    }

    fn from(memory: [Option<u8>; 2]) -> Self {
        Self {
            memory,
            low: None,
            high: None,
        }
    }

    fn addval(&mut self, value: u8) -> bool {
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
            return false;
        }
        true
    }

    fn considerable(&self) -> bool {
        self.memory.into_iter().all(|x| x.is_some())
    }

    fn wipe(&mut self) {
        self.memory[0] = None;
        self.memory[1] = None;
    }
}

struct Bots {
    bots: HashMap<Location, Bot>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Location {
    bottype: Bottype,
    id: u8,
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
                    if let Some(x) = ret.bots.get_mut(&Location {
                        bottype: Bottype::Bot,
                        id: *bot,
                    }) {
                        x.low = Some(*low);
                        x.high = Some(*high);
                    } else {
                        ret.bots.insert(
                            Location {
                                bottype: Bottype::Bot,
                                id: *bot,
                            },
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

    fn iteration(&mut self) -> HashMap<(u8, u8), Location> {
        let consider = self
            .bots
            .iter()
            .filter(|(_, v)| v.considerable())
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();
        let ret = consider
            .iter()
            .map(|(k, v)| ((v.memory[0].unwrap(), v.memory[1].unwrap()), *k))
            .collect::<HashMap<_, _>>();
        for (curr, bot) in consider {
            if bot.low.unwrap().bottype == Bottype::Output {
                self.bots.insert(bot.low.unwrap(), Bot::default());
            }
            if bot.high.unwrap().bottype == Bottype::Output {
                self.bots.insert(bot.high.unwrap(), Bot::default());
            }
            if self.bots.get_mut(&bot.low.unwrap()).unwrap().count() == 2
                || self.bots.get_mut(&bot.high.unwrap()).unwrap().count() == 2
            {
                continue;
            }
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
