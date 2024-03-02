use std::{cmp, collections::{HashMap, VecDeque}, io::{self, Write}};

fn main() {
    const SALT: &[u8] = b"abc";
    let mut triplets = VecDeque::new();
    let mut found = 0;
    for i in 0.. {
        let digest = md5::compute([SALT, i.to_string().as_bytes()].concat())
            .0
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
        let counts = HashMap::new();
        for c in digest.chars() {
            if let Some(x) = counts.get_mut(&c) {
                *x+=1;
            } else {
                counts.insert(c, 1);
            }
        }
        for j in counts.iter().filter(|(a,b)|b>=5) {
            for x in triplets {
                if x.1 == a {
                    found += 1;
                    if found == 64
                }
            }
        }
        if max >= 5 {
            found += triplets.len();
            if found >= 64 {
                // println!("{found}");
                // println!("{triplets:?}");
                // println!(
                //     "{:?}",
                //     triplets.get(triplets.len() - (found - 63)).unwrap()
                // );
                return;
            }
            println!("{i}\n{triplets:?}");
            triplets.clear();
        } else if max >= 3 {
            triplets.push_back(i);
        }
        while let Some(x) = triplets.get(0) {
            if i == x.0 + 1000 {
                triplets.pop_front();
            } else {
                break;
            }
        }
        // println!("{triplets:?}");
        // println!("{found}");
        // print!("{i}");
        // io::stdout().flush();
        // let mut tmp = String::default();
        // io::stdin().read_line(&mut tmp);
    }
}
