fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let mut rows: Vec<usize> = Vec::new();
    let mut columns: Vec<usize> = Vec::new();
    let mut withrows: Vec<Vec<bool>> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        withrows.push(line.clone());
        if line.iter().all(|b| !b) {
            rows.push(i);
        }
    }

    let mut rotated: Vec<Vec<bool>> = Vec::new();
    let mut tmp: Vec<bool> = Vec::new();
    tmp.resize(withrows.len(), false);
    rotated.resize(withrows[0].len(), tmp);
    for i in 0..withrows[0].len() {
        for j in 0..withrows.len() {
            rotated[i][j] = withrows[j][i];
        }
    }

    let mut withcolumns: Vec<Vec<bool>> = Vec::new();
    for (i, line) in rotated.iter().enumerate() {
        withcolumns.push(line.clone());
        if line.iter().all(|b| !b) {
            columns.push(i);
        }
    }
    let mut rotated: Vec<Vec<bool>> = Vec::new();
    let mut tmp: Vec<bool> = Vec::new();

    tmp.resize(withcolumns.len(), false);
    rotated.resize(withcolumns[0].len(), tmp);
    for i in 0..withcolumns[0].len() {
        for j in 0..withcolumns.len() {
            rotated[i][j] = withcolumns[j][i];
        }
    }

    let mut positions: Vec<(usize, usize)> = Vec::new();
    for (i, line) in rotated.iter().enumerate() {
        for (j, pos) in line.iter().enumerate() {
            if *pos {
                positions.push((i, j));
            }
        }
    }
    let mut sum = 0;
    let increment: u64 = 1000000;
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            if positions[i].0 < positions[j].0 {
                for k in positions[i].0..positions[j].0 {
                    if rows.contains(&k) {
                        sum += increment;
                    } else {
                        sum += 1;
                    }
                }
            } else {
                for k in positions[j].0..positions[i].0 {
                    if rows.contains(&k) {
                        sum += increment;
                    } else {
                        sum += 1;
                    }
                }
            }
            if positions[i].1 < positions[j].1 {
                for k in positions[i].1..positions[j].1 {
                    if columns.contains(&k) {
                        sum += increment;
                    } else {
                        sum += 1;
                    }
                }
            } else {
                for k in positions[j].1..positions[i].1 {
                    if columns.contains(&k) {
                        sum += increment;
                    } else {
                        sum += 1;
                    }
                }
            }
        }
    }
    println!("{sum}")
}
