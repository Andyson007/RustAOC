fn main() {
    let input = include_str!("../../input").lines().next().unwrap();
    let mut ans = None;
    for a in 0..100 {
        for b in 0..100 {
            let mut values = input
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            values[1] = a;
            values[2] = b;
            if the_thing(values) == 19690720 {
                ans = Some((a, b));
            }
        }
    }
    println!("{:?}", ans.unwrap());
}

fn the_thing(mut values: Vec<i32>) -> i32 {
    let mut pos = 0;
    loop {
        if values[pos] == 99 {
            return values[0];
        }
        operate(pos, &mut values);
        pos += 4;
    }
}

fn operate(pos: usize, values: &mut [i32]) {
    values[values[pos + 3] as usize] = match values[pos] {
        1 => values[values[pos + 1] as usize] + values[values[pos + 2] as usize],
        2 => values[values[pos + 1] as usize] * values[values[pos + 2] as usize],
        _ => unimplemented!(),
    };
}
