use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let parts = line.split("->").collect::<Vec<&str>>();
            let left = parts[0].trim();
            let right = parts[1].split(",").map(|x| x.trim()).collect::<Vec<&str>>();
            (left, right)
        })
        .collect::<HashMap<&str, Vec<&str>>>();
    let broadcaster = input.get("broadcaster").unwrap().clone();
    input.remove("broadcaster");
    let mut input = input
        .iter()
        .map(|(k, v)| {
            let typo = k.chars().nth(0).unwrap();
            let k = &k[1..];
            match typo {
                '%' => return (k, (typo, v.clone(), false, HashMap::new())),
                '&' => return (k, (typo, v.clone(), false, HashMap::new())),
                _ => unreachable!(),
            }
        })
        .collect::<HashMap<&str, (char, Vec<&str>, bool, HashMap<&str, bool>)>>();
    for (k, v) in input.clone() {
        for a in v.1 {
            if let Some(x) = input.get(&a) {
                if x.0 == '&' {
                    input.get_mut(&a).unwrap().3.insert(k, false);
                }
            } else {
              input.remove(&a);
            }
        }
    }
    let (mut highs, mut lows): (u64, u64) = (0, 0);
    for _ in 0..1000 {
        lows += broadcaster.len() as u64 + 1;
        let mut signals: Vec<(&str, bool, &str)> = broadcaster
            .iter()
            .map(|b| (*b, false, "button"))
            .collect::<Vec<(&str, bool, &str)>>();
        while signals.len() != 0 {
            let mut newsignals: Vec<(&str, bool, &str)> = Vec::new();
            for signal in signals {
                if !input.contains_key(&signal.0) {
                  continue;
                }
                let module = input.get_mut(&signal.0).unwrap();
                match module.0 {
                    '%' => {
                        if !signal.1 {
                            module.2 = !module.2;
                            for dest in module.1.clone() {
                                newsignals.push((dest, module.2, signal.0));
                            }
                        }
                    }
                    '&' => {
                        *module.3.get_mut(&signal.2).unwrap() = signal.1;
                        let tosend = !module.3.iter().all(|x| *x.1);
                        for dest in module.1.clone() {
                            newsignals.push((dest, tosend, signal.0));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            for signal in newsignals.clone() {
                if signal.1 {
                    highs += 1;
                } else {
                    lows += 1;
                }
            }
            signals = newsignals;
        }
    }
    println!("{highs} * {lows} = {}", highs*lows);
}
