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
    let mut poses = raw
        .lines()
        .map(|line| line.chars().map(|x| x == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start_dir = (0, -1);
    let start_pos = raw
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, (x, y))))
        .find(|(c, _)| *c == '^')
        .unwrap()
        .1;

    let mut pos = start_pos;
    let mut dir = start_dir;
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
        if poses[new_y][new_x] {
            dir = rotate(dir);
            continue;
        }
        pos = (new_x, new_y);
        visited.insert(pos);
    }
    let mut loop_counter = 0;
    for location in visited {
        poses[location.1][location.0] = true;

        let mut pos = start_pos;
        let mut dir = start_dir;
        let mut visited = HashSet::from([(pos, dir)]);
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
            if poses[new_y][new_x] {
                dir = rotate(dir);
                continue;
            }
            pos = (new_x, new_y);
            if !visited.insert((pos, dir)) {
                loop_counter += 1;
                break;
            }
        }

        poses[location.1][location.0] = false;
    }
    loop_counter
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
        assert_eq!(ans, 6)
    }
}
