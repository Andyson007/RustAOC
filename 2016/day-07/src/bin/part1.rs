fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .filter(|line| {
            let mut b = false;
            let mut found = (false, false);
            for arr in line.chars().collect::<Vec<char>>().windows(4) {
                if arr[0] == '[' {
                    b = true;
                } else if arr[0] == ']' {
                    b = false
                } else if arr[0] == arr[3] && arr[1] == arr[2] && arr[0] != arr[1] {
                    if b {
                        found.0 = true;
                    } else {
                        found.1 = true;
                    }
                }
            }
            found.1 && !found.0
        })
        .count();
    println!("{ans}");
}
