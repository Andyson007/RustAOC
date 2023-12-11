use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let moves = input.lines().nth(0).unwrap();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let loc = line.chars().take(3).collect::<String>();
        let l = line.chars().skip(7).take(3).collect::<String>();
        let r = line.chars().skip(12).take(3).collect::<String>();
        if map.contains_key(&loc) {}
        map.insert(loc, (l, r));
    });

    let mut count: u32 = 0;
    let mut curr = String::from("AAA");
    loop {
        for m in moves.chars() {
            println!("{curr} {m}");
            if curr == "ZZZ" {
                println!("{count}");
                return ();
            }
            curr = match m {
                'L' => map.get(&curr).unwrap().0.clone(),
                'R' => map.get(&curr).unwrap().1.clone(),
                _ => unreachable!(),
            };
            count += 1;
        }
    }
}
