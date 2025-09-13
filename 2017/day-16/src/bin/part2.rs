#[derive(Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn main() {
    let input = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|m| match m.chars().next().unwrap() {
            's' => Move::Spin(
                m.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            ),
            'x' => {
                let split = m
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .split('/')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                Move::Exchange(split[0], split[1])
            }
            'p' => Move::Partner(m.chars().nth(1).unwrap(), m.chars().nth(3).unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<Move>>();
    let len = 16;
    let mut arr = ('a'..).take(len).collect::<Vec<char>>();
    for m in &input {
        match m {
            Move::Exchange(a, b) => {
                arr.swap(*a, *b);
            }
            Move::Partner(a, b) => {
                let (a, b) = (
                    arr.iter().position(|x| *x == *a).unwrap(),
                    arr.iter().position(|x| *x == *b).unwrap(),
                );
                arr.swap(a, b);
            }
            Move::Spin(x) => {
                let mut end = Vec::new();
                for _ in 0..*x {
                    end.push(arr.pop().unwrap());
                }
                for elem in end {
                    arr.insert(0, elem);
                }
            }
        }
    }

    let start = ('a'..).take(len).collect::<Vec<char>>();
    let mut arr = ('a'..).take(len).collect::<Vec<char>>();
    let mut repeat = 0;

    loop {
        repeat += 1;
        for m in &input {
            match m {
                Move::Exchange(a, b) => {
                    arr.swap(*a, *b);
                }
                Move::Partner(a, b) => {
                    let (a, b) = (
                        arr.iter().position(|x| *x == *a).unwrap(),
                        arr.iter().position(|x| *x == *b).unwrap(),
                    );
                    arr.swap(a, b);
                }
                Move::Spin(x) => {
                    let mut end = Vec::new();
                    for _ in 0..*x {
                        end.push(arr.pop().unwrap());
                    }
                    for elem in end {
                        arr.insert(0, elem);
                    }
                }
            }
        }
        if arr == start {
            break;
        }
    }
    for _ in 0..1000000000 % repeat {
        for m in &input {
            match m {
                Move::Exchange(a, b) => {
                    arr.swap(*a, *b);
                }
                Move::Partner(a, b) => {
                    let (a, b) = (
                        arr.iter().position(|x| *x == *a).unwrap(),
                        arr.iter().position(|x| *x == *b).unwrap(),
                    );
                    arr.swap(a, b);
                }
                Move::Spin(x) => {
                    let mut end = Vec::new();
                    for _ in 0..*x {
                        end.push(arr.pop().unwrap());
                    }
                    for elem in end {
                        arr.insert(0, elem);
                    }
                }
            }
        }
    }
    let ans = arr.iter().collect::<String>();
    println!("{ans}");
}
