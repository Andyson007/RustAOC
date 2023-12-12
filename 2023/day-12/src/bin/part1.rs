fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .enumerate()
        .map(|(index, line)| {
            println!("{index}");
            let mut ret = 0;
            let content = line.split(" ").nth(0).unwrap();
            let arrangements = line
                .split(" ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let count = content.matches("?").count();
            for mut i in 0..(1 << count) {
                let mut newstr = String::from("");
                for c in content.chars() {
                    if c == '?' {
                        if i & 1 == 1 {
                            newstr.push('#');
                        } else {
                            newstr.push('.');
                        }
                        i >>= 1;
                    } else {
                        newstr.push(c);
                    }
                }
                let mut count = 0;
                let mut newarrangements: Vec<u32> = Vec::new();
                for c in newstr.chars() {
                    if c == '.' {
                        if count != 0 {
                            newarrangements.push(count);
                        }
                        count = 0;
                    } else {
                        count += 1;
                    }
                }
                if count != 0 {
                    newarrangements.push(count);
                }
                if newarrangements == arrangements {
                    ret += 1;
                }
            }
            ret
        })
        .sum::<u32>();
    println!("{ans}");
}
