fn main() {
    let (column, row) = (3010, 3019);
    let toprow = row + column - 1;
    let toptriangle = toprow * (toprow + 1) / 2;
    let index = toptriangle - column + 1;
    let mut val: u128 = 20151125;
    for _ in 0..index - 1 {
        val = (val * 252533) % 33554393;
    }
    println!("{val}");
}
