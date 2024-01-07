use std::collections::HashSet;

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let input = include_str!("../../input.txt")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start = (i, j);
                        b'a'
                    }
                    'E' => {
                        end = (i, j);
                        b'z'
                    }
                    x => x as u8,
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let mut count = 0;
    let mut tocheck = Vec::from([start]);
    let mut visited = HashSet::new();
    'outer: loop {
        count += 1;
        let mut next = Vec::new();
        for coord in tocheck {
            visited.insert(coord);
            for dir in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let newcoord = (coord.0 as i8 + dir.0, coord.1 as i8 + dir.1);
                if newcoord.0 < 0
                    || newcoord.1 < 0
                    || newcoord.0 == input.len() as i8
                    || newcoord.1 == input[newcoord.0 as usize].len() as i8
                {
                    continue;
                }
                let newcoord = (newcoord.0 as usize, newcoord.1 as usize);
                if input[newcoord.0][newcoord.1] <= input[coord.0][coord.1] + 1 {
                    if newcoord == end {
                        break 'outer;
                    }
                    if !visited.contains(&newcoord) {
                        next.push(newcoord);
                    }
                }
            }
        }
        tocheck = next;
        println!("{count}");
    }
    println!("{count}");
}
