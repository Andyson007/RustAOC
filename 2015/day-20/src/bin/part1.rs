fn main() {
    let input: u32 = 33100000 / 10;

    for i in (0..).step_by(20) {
        let mut score = 0;
        for j in 1..=i {
            if i % j == 0 {
                score += j;
            }
        }
        if score>= input {
          println!("{i}");
          break;
        }
    }
}
