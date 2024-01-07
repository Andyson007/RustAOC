fn main() {
    let input: u32 = 33100000;

    let mut max = 0;
    for i in (0..).step_by(20) {
        let mut score = 0;
        for j in 1..=i {
            if i <= 50 * j {
                if i % j == 0 {
                    score += j;
                }
            }
        }
        // println!("{score}");
        if score > max {
            println!("{i}: {}", score*11);
            max = score;
            if score * 11 >= input {
                println!("{i}");
                break;
            }
        }
    }
}
