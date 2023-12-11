fn main() {
    let input = include_str!("../../input.txt");
    let len = input.lines().count();
    let bitlen = input.lines().nth(0).unwrap().len();
    let bitcount = input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .inspect(|val| println!("{val}"))
        .fold([0usize; 16], |mut sum, val| {
            for i in 0..16 {
                sum[i] += if (val & (1 << i)) == 0 { 0 } else { 1 };
            }
            sum
        })
        .iter()
        .take(bitlen)
        .map(|bit| (*bit * 2) < len)
        .rev()
        .collect::<Vec<bool>>();
    let mut val: u32 = 0;
    for a in bitcount {
        val <<= 1;
        val |= if a { 1 } else { 0 };
    }
    let ans =  val*(!val & ((1 << bitlen) - 1));
    println!("{ans}");
}
