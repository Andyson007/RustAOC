fn main() {
    let input = include_str!("../../input").lines().next().unwrap();
    let mut values = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    values[1] = 12;
    values[2] = 2;
    let ans = the_thing(values);
    println!("{ans}");
}

fn the_thing(mut values: Vec<i32>) -> i32 {
    let mut pos = 0;
    loop {
        if values[pos] == 99 {
            return values[0];
        }
        operate(pos, &mut values);
        pos += 4;

        eprintln!("{values:?}");
    }
}

fn operate(pos: usize, values: &mut [i32]) {
    values[values[pos + 3] as usize] = match values[pos] {
        1 => values[values[pos + 1] as usize] + values[values[pos + 2] as usize],
        2 => values[values[pos + 1] as usize] * values[values[pos + 2] as usize],
        _ => unimplemented!(),
    };
}

#[test]
fn part1_test() {
    let input = include_str!("../../example").lines().next().unwrap();
    let values = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    let ans = the_thing(values);
    assert_eq!(ans, 3500);
}
