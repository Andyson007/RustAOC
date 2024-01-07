use std::collections::HashSet;

fn main() {
    let mut input = include_str!("../../input.txt").split("\r\n\r\n");
    //     let mut input = "e => H
    // e => O
    // H => HO
    // H => OH
    // O => HH
    //
    // HOHOHO"
    //         .split("\n\n");
    let mappings = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.split(" => ").collect::<Vec<&str>>();
            (line[1], line[0])
        })
        .collect::<HashSet<(&str, &str)>>();
    let molecule = input.next().unwrap().lines().nth(0).unwrap();
    let mut molecules = HashSet::from([molecule.to_string()]);
    let mut depth = 0;
    while !molecules.contains("e") {
      println!("{depth} {}",molecules.len());
        depth += 1;
        let mut new = HashSet::new();
        for molecule in molecules {
            for map in &mappings {
                let split = molecule.split(map.0).collect::<Vec<&str>>();
                for i in 1..split.len() {
                    let mut to_push = split
                        .iter()
                        .take(i)
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(map.0);
                    to_push.push_str(map.1);
                    to_push.push_str(
                        split
                            .iter()
                            .skip(i)
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(map.0)
                            .as_str(),
                    );
                    new.insert(to_push.clone());
                }
            }
        }
        molecules = new;
    }
    println!("{depth}")
}
