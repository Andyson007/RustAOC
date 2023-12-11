fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let mut emptyrow: Vec<bool> = Vec::new();
    emptyrow.resize(input[0].len(), false);
    let mut withrows: Vec<Vec<bool>> = Vec::new();
    for line in input {
        withrows.push(line.clone());
        if line.iter().all(|b| !b) {
            withrows.push(emptyrow.clone());
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

    let mut emptyrow: Vec<bool> = Vec::new();
    emptyrow.resize(rotated[0].len(), false);
    let mut withcolumns: Vec<Vec<bool>> = Vec::new();
    for line in rotated {
        withcolumns.push(line.clone());
        if line.iter().all(|b| !b) {
            withcolumns.push(emptyrow.clone());
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

    for a in rotated.clone() {
        for b in a {
            print!("{}", if b { 1 } else { 0 });
        }
        println!();
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
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            sum += positions[i].0.abs_diff(positions[j].0);
            sum += positions[i].1.abs_diff(positions[j].1);
        }
    }
    println!("{sum}");
}
