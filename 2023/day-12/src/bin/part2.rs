use std::collections::HashMap;

fn main() {
    let mul = 5;
    let input = include_str!("../../input.txt");
    let ans = input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let originalcontent = line.split(" ").nth(0).unwrap();
            let originalarrengements = line
                .split(" ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut arrangements: Vec<usize> = Vec::new();
            for _ in 0..mul {
                for a in originalarrengements.clone() {
                    arrangements.push(a);
                }
            }

            let mut content = String::from("");
            for _ in 0..mul {
                content.push_str(originalcontent);
                content.push('?');
            }
            content.pop();

            solve(content, arrangements)
        })
        .sum::<u64>();
    println!("{ans}");
}

fn solve(content: String, arrangements: Vec<usize>) -> u64 {
    let mut to_solve: HashMap<(String, Vec<usize>), u64> = HashMap::new();
    // let mut to_solve: Vec<(String, Vec<usize>)> = Vec::new();
    to_solve.insert((content.trim_matches('.').to_string(), arrangements), 1);
    // to_solve.push((
    //     "????.#...#...?????.#...#...?????.#...#...?????.#...#".to_string(),
    //     vec![4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1],
    // ));
    let mut ret = 0;
    while to_solve.len() != 0 {
        // 'outer: for i in 0..40 {
        let mut newsolve: HashMap<(String, Vec<usize>), u64> = HashMap::new();
        // let mut newsolve: Vec<(String, Vec<usize>)> = Vec::new();
        'outer: for ((mut content, mut arrangements), count) in to_solve {
            if arrangements.iter().map(|x| *x).sum::<usize>()
                > (content.matches('?').count() + content.matches('#').count())
            {
                continue;
            }
            if arrangements.len() == 0 {
                if content.matches('#').count() == 0 {
                    ret += count;
                }
                continue;
            }
            if content.ends_with('#') {
                for _ in 0..*arrangements.last().unwrap() {
                    if !content.ends_with('.') {
                        content.pop();
                    } else {
                        continue 'outer;
                    }
                }
                if !content.ends_with('#') {
                    content.pop();
                    arrangements.pop();
                    let toadd = (content.trim_end_matches('.').to_string(), arrangements);
                    if newsolve.contains_key(&toadd) {
                        *newsolve.get_mut(&toadd).unwrap() += count;
                    } else {
                        newsolve.insert(toadd, count);
                    }
                }
            } else if content.ends_with('?') {
                let toadd = (
                    content[0..(content.len() - 1)]
                        .to_string()
                        .trim_matches('.')
                        .to_string(),
                    arrangements.clone(),
                );
                if let Some(x) = newsolve.get_mut(&toadd) {
                    *x += count;
                } else {
                    newsolve.insert(toadd, count);
                }
                for _ in 0..*arrangements.last().unwrap() {
                    if !content.ends_with('.') {
                        content.pop();
                    } else {
                        continue 'outer;
                    }
                }
                if !content.ends_with('#') {
                    content.pop();
                    arrangements.pop();
                    let toadd = (content.trim_end_matches('.').to_string(), arrangements);
                    // newsolve.push(toadd);
                    if newsolve.contains_key(&toadd) {
                        *newsolve.get_mut(&toadd).unwrap() += count;
                    } else {
                        newsolve.insert(toadd, count);
                    }
                }
            } else if content.ends_with('.') {
                let toadd = (content.trim_end_matches('.').to_string(), arrangements);
                // newsolve.push(toadd);
                if newsolve.contains_key(&toadd) {
                    *newsolve.get_mut(&toadd).unwrap() += count;
                } else {
                    newsolve.insert(toadd, count);
                }
            } else {
                if arrangements.len() == 0 {
                    ret += 1;
                }
            }
        }
        to_solve = newsolve;
    }
    ret
}
