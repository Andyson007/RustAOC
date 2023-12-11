fn main() {
    let mut count = 0;
    for i in 158126..=624574 {
        if i.to_string()
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .all(|arr| arr[0] <= arr[1])
            && i.to_string()
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .any(|arr| arr[0] == arr[1])
        {
            count += 1;
        }
    }
    print!("{count}");
}
