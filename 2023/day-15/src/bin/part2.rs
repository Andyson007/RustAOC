use std::array;

fn main() {
    let mut lenses: [Vec<(String, u8)>; 256] = array::from_fn(|_| Vec::new());
    for s in include_str!("../../input.txt").split(",") {
        if s.matches("=").count() == 1 {
            let toadd = s.trim().split("=").nth(0).unwrap();
            let pos = toadd
                .chars()
                .fold(0, |sum, curr| (sum + curr as u8) * 17)
                as usize;
            let value = s.trim().split("=").nth(1).unwrap().parse::<u8>().unwrap();
            let index = lenses[pos].iter().position(|v| v.0 == toadd);
            if let Some(index) = index {
                lenses[pos][index].1 = value;
            } else {
                lenses[pos].push((s.trim().split("=").nth(0).unwrap().to_string(), value));
            }
        } else if s.matches("-").count() == 1 {
            let toadd = s.trim().split("-").nth(0).unwrap();
            let pos = toadd
                .chars()
                .fold(0, |sum, curr| (sum + curr as u8) * 17)
                as usize;
            let index = lenses[pos].iter().position(|v| v.0 == toadd);
            if let Some(index) = index {
                lenses[pos].remove(index);
            }
        } else {
            unreachable!()
        }
    }
    for lens in lenses.clone() {
        println!("{lens:?}");
    }
    let ans = lenses
        .iter()
        .enumerate()
        .map(|(i, line)| {
            (i + 1) as u32
                * line
                    .iter()
                    .enumerate()
                    .map(|(i, val)| (i as u32 + 1) * val.1 as u32)
                    .sum::<u32>()
        })
        .sum::<u32>();
    println!("{ans}");
}
