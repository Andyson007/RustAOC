#![feature(linked_list_retain)]
use std::{collections::LinkedList, fmt::format};

const DIGITS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

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
        let mut digest = md5::compute([SALT, i.to_string().as_bytes()].concat()).0;
        for _ in 0..2016 {
            let a: [u8; 32] = digest
                .iter()
                .flat_map(|x| [DIGITS[*x as usize / 16], DIGITS[*x as usize % 16]])
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            digest = *md5::compute(a);
        }
        let mut count = -1;
        let mut prev = '\0';
        let mut repeating_char = None;
        for curr in digest
            .into_iter()
            .flat_map(|x| format!("{:02x}", x).chars().collect::<Vec<char>>())
            .chain(['\0'])
        {
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
    for (i, value) in values.iter().enumerate() {
        println!("{i}: {value}");
    }
    println!("{}", values[63]);
}

