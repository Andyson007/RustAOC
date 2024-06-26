#![feature(linked_list_retain)]
use std::collections::LinkedList;

fn main() {
    const SALT: &[u8] = b"ihaygndm";
    let mut triplets = LinkedList::new();
    let mut found = 0;
    let mut limit = None;
    let mut values = Vec::new();
    for i in 0.. {
        if limit.is_some_and(|x| i > x) {
            break;
        }
        let digest = md5::compute([SALT, i.to_string().as_bytes()].concat())
            .0
            .into_iter()
            .flat_map(|x| format!("{:02x}", x).chars().collect::<Vec<char>>());
        let mut count = -1;
        let mut prev = '\0';
        let mut repeating_char = None;
        for curr in digest.into_iter().chain(['\0']) {
            if curr == prev {
                count += 1;
            } else {
                if count >= 3 && repeating_char.is_none() {
                    repeating_char = Some(prev);
                }
                if count >= 5 {
                    triplets.retain(|(x, y): &(_, i32)| {
                        if *x == prev {
                            found += 1;
                            if found == 64 {
                                limit = Some(y + 1000);
                            }
                            values.push(y.to_owned());
                            false
                        } else {
                            true
                        }
                    })
                }

                count = 1;
                prev = curr;
            }
        }
        if let Some(x) = repeating_char {
            triplets.push_back((x, i));
        }
        if triplets.front().is_some_and(|(_, b)| *b == i - 1000) {
            triplets.pop_front();
        }
    }
    values.sort();
    // for (i, value) in values.into_iter().enumerate() {
    println!("{}", values[63])
    // }
}
