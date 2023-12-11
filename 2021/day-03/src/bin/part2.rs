fn main() {
    let input = include_str!("../../input.txt");
    let bitlen = input.lines().nth(0).unwrap().len();
    let mut oxygen = input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u16>>();
    let mut scrubber = oxygen.clone();
    let mut i = bitlen as i8 - 1;
    while oxygen.len() != 1 {
        let count = oxygen.iter().filter(|x| (*x >> i) & 1 == 1).count();
        oxygen = oxygen
            .iter()
            .map(|x| *x)
            .filter(|x| (x >> i) & 1 == (if 2 * count < oxygen.len() { 0 } else { 1 }))
            .collect::<Vec<u16>>();
        i -= 1;
    }
    i = bitlen as i8 - 1;
    while scrubber.len() != 1 {
        let count = scrubber.iter().filter(|x| (*x >> i) & 1 != 1).count();
        scrubber = scrubber
            .iter()
            .map(|x| *x)
            .filter(|x| (x >> i) & 1 == (if 2 * count <= scrubber.len() { 0 } else { 1 }))
            .collect::<Vec<u16>>();
        i -= 1;
    }
    let ans = scrubber[0] as u32 *oxygen[0] as u32 ;
    println!("{ans}");
}
