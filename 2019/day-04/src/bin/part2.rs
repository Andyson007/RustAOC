fn main() {
    let mut count = 0;
    for i in 158126..=624574 {
        let a = i.to_string().chars().collect::<Vec<char>>();

        if a.windows(2).all(|arr| arr[0] <= arr[1]) && a.windows(2).any(|arr| arr[0] == arr[1]) {
            let matches = a
                .windows(2)
                .map(|arr| arr[0] == arr[1])
                .collect::<Vec<bool>>();
            if (matches[0] && !matches[1])
                || (!matches[matches.len() - 2] && matches[matches.len() - 1])
            {
                count += 1;
            } else if matches.windows(3).any(|arr| !arr[0] && arr[1] && !arr[2]) {
                count += 1;
            }
        }
    }
    print!("{count}");
}
