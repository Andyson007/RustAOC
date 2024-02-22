fn main() {
    let input = include_str!("../../input")
        .lines()
        .map(|x| x.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut count = 0;
    for x in 0..input.len() as i32 {
        'inner: for y in 0..input[x as usize].len() as i32 {
            let curr = input[x as usize][y as usize];
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let newpos = ((x + dir.0), (y + dir.1));
                if newpos.0 >= 0
                    && newpos.0 < input.len() as i32
                    && newpos.1 >= 0
                    && newpos.1 < input[x as usize].len() as i32
                {
                    let newpos = (newpos.0 as usize, newpos.1 as usize);
                    if input[newpos.0][newpos.1] <= curr {
                        continue 'inner;
                    }
                }
            }
            count += ((curr ^ 48) + 1) as usize;
            println!("{x} {y}");
        }
    }
    println!("{count}");
}
