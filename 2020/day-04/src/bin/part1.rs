fn main() {
    let requiredfields = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    let ans = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|line| {
            let fields = line
                .split_whitespace()
                .map(|a| a.split(":").nth(0).unwrap().to_string())
                .collect::<Vec<String>>();
            for field in requiredfields.clone() {
                if !fields.contains(&field.to_string()) {
                    return false;
                }
            }
            true
        })
        .fold(0u32, |sum, curr| sum + if curr { 1 } else { 0 });
    println!("{ans}");
}
