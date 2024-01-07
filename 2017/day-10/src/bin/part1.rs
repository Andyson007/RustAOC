fn main() {
    let input = [14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244];

    let mut current: u8 = 0;
    let mut skip: u8 = 0;
    let mut list = (0..256).collect::<Vec<usize>>();

    for i in input {
        for j in 0..i / 2 {
            list.swap((current + j) as usize, (current + i - j - 1) as usize);
        }
        current += i + skip;
        skip += 1;
    }
    println!("{}", list[0] * list[1]);
}
