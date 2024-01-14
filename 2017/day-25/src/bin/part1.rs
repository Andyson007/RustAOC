use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Action {
    tape: bool,
    dir: bool,
    next: char,
}

#[derive(Debug)]
struct State {
    On: Action,
    Off: Action,
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut state = input.lines().nth(0).unwrap().chars().nth_back(1).unwrap();
    let iters = input
        .chars()
        .skip_while(|x| !x.is_digit(10))
        .take_while(|x| x.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let states = input
        .split("\r\n\r\n")
        .skip(1)
        .map(|x| {
            let state = x.lines().nth(0).unwrap().chars().nth_back(1).unwrap();
            let commands = x.lines().filter(|x| x.contains("-")).collect::<Vec<&str>>();
            (
                state,
                State {
                    On: Action {
                        tape: commands[3].chars().nth_back(1).unwrap() == '1',
                        dir: commands[4].chars().nth_back(2).unwrap() == 'h',
                        next: commands[5].chars().nth_back(1).unwrap(),
                    },
                    Off: Action {
                        tape: commands[0].chars().nth_back(1).unwrap() == '1',
                        dir: commands[1].chars().nth_back(2).unwrap() == 'h',
                        next: commands[2].chars().nth_back(1).unwrap(),
                    },
                },
            )
        })
        .collect::<HashMap<char, State>>();
    let mut tape = HashSet::new();
    let mut pos = 0;
    for _ in 0..iters {
        let curr = if tape.contains(&pos) {
            &states.get(&state).unwrap().On
        } else {
            &states.get(&state).unwrap().Off
        };
        if curr.tape {
            tape.insert(pos);
        } else {
            tape.remove(&pos);
        }
        if curr.dir {
            pos += 1;
        } else {
            pos -= 1;
        }
        state = curr.next;
    }
    println!("{}", tape.len());
}
