use std::collections::HashSet;

fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .filter(|line| {
            let mut outside = HashSet::new();
            let mut inside = HashSet::new();
            let mut curr = Vec::new();
            for c in line.chars() {
                match c {
                    '[' => {
                        outside.insert(curr.clone());
                        curr.clear();
                    }
                    ']' => {
                        inside.insert(curr.clone());
                        curr.clear();
                    }
                    x => curr.push(x),
                }
            }
            outside.insert(curr.clone());
            for ins in inside {
              for arr in ins.windows(3) {
                if arr[0] == arr[2] && arr[0] != arr[1] {
                  for out in &outside {
                    for arr2 in out.windows(3) {
                      if arr[0]==arr2[1] && arr[1] == arr2[0] && arr[1]==arr2[2] {
                        return true;
                      }
                    }
                  }
                }
              }
            }
            false
        })
        .count();
    println!("{ans}");
}
