fn main() {
    let input = include_str!("../../input.txt");
    let references = input
        .lines()
        .filter(|line| line.matches("->").count() != 0)
        .map(|line| {
            let mut w = line.split_whitespace();

            (
                String::from(w.nth(0).unwrap()),
                w.skip(2)
                    .map(|x| {
                        if x.chars().rev().nth(0).unwrap() == ',' {
                            return x
                                .chars()
                                .take(x.chars().count() - 1)
                                .collect::<Vec<char>>()
                                .into_iter()
                                .collect::<String>();
                        }
                        String::from(x)
                    })
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<(String, Vec<String>)>>();
    let mut candidates = references.iter().map(|(a, _)| a.clone()).collect::<Vec<String>>();
    for (_, disc) in references {
        for a in disc {
            for (i, c) in candidates.clone().iter().enumerate() {
                if a == *c {
                    candidates.remove(i);
                }
            }
        }
    }
    println!("{}", candidates[0]);
}
