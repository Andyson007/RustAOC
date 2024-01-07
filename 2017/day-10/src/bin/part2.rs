fn main() {
    let input = "14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244"
        .chars()
        .map(|c| c as u8)
        .chain([17, 31, 73, 47, 23])
        .collect::<Vec<u8>>();
    println!("{input:?}");
    let mut current: u8 = 0;
    let mut skip: u8 = 0;
    let mut list = (0..=255).collect::<Vec<u8>>();
    for _ in 0..64 {
        println!("{current} {skip}");
        for i in &input {
            for j in 0..i / 2 {
                list.swap((current + j) as usize, (current + i - j - 1) as usize);
            }
            current += i + skip;
            skip += 1;
        }
    }
    let dense = list
        .chunks(16)
        .map(|x| x.iter().fold(0, |a, b| a ^ b))
        .map(|c| format!("{c:02x}"))
        .collect::<String>();
    println!("{dense}");
}
