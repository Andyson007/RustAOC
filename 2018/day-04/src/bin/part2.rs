use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    date: String,
    hour: u64,
    minute: u64,
    action: Action,
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Start(u64),
    Sleep,
    Wake,
}

fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line
                .chars()
                .skip(1)
                .collect::<String>()
                .split("] ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let action = match split[1].split_whitespace().nth(1).unwrap() {
                "up" => Action::Wake,
                "asleep" => Action::Sleep,
                x => Action::Start(
                    x.chars()
                        .skip(1)
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap(),
                ),
            };
            let time = split[0].split_whitespace().collect::<Vec<&str>>();
            let clock = time[1]
                .split(":")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            Line {
                date: time[0].to_string(),
                hour: clock[0],
                minute: clock[1],
                action,
            }
        })
        .collect::<Vec<Line>>();
    input.sort_by(|a, b| {
        a.date
            .cmp(&b.date)
            .then(a.hour.cmp(&b.hour))
            .then(a.minute.cmp(&b.minute))
    });
    let mut counts = HashMap::new();
    let mut sleep = 0;
    let mut guard = 0;

    for line in &input {
        match line.action {
            Action::Start(x) => {
                guard = x;
                if !counts.contains_key(&guard) {
                    counts.insert(guard, [0; 60]);
                }
            }
            Action::Sleep => sleep = line.minute,
            Action::Wake => {
                for i in sleep..line.minute {
                    counts.get_mut(&guard).unwrap()[i as usize] += 1;
                }
            }
        }
    }
    let ans = counts
        .iter()
        .map(|(k, v)| {
            let (a, b) = v.iter().enumerate().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            (k * a as u64, b)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    println!("{ans}");
}
