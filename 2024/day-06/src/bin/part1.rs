use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let size = (
        raw.lines().count(),
        raw.lines().next().unwrap().chars().count(),
    );
    let poses = raw
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, x)| *x == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();
    let mut dir = (0, -1);
    let mut pos = raw
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, (x, y))))
        .find(|(c, _)| *c == '^')
        .unwrap()
        .1;
    let mut visited = HashSet::from([pos]);
    loop {
        let new_x = dir.0 + pos.0 as isize;
        let new_y = dir.1 + pos.1 as isize;
        if new_x < 0 || new_y < 0 {
            break;
        }
        let new_x = usize::try_from(new_x).unwrap();
        let new_y = usize::try_from(new_y).unwrap();
        if new_x >= size.0 || new_y >= size.1 {
            break;
        }
        if poses.contains(&(new_x, new_y)) {
            dir = rotate(dir);
            continue;
        }
        pos = (new_x, new_y);
        visited.insert(pos);
    }
    visited.len()
}

fn rotate((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 41)
    }
}
