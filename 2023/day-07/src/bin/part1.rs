use std::cmp::Ordering;
fn main() {
    let input = include_str!("../../input.txt");
    let mut lines: Vec<(&str, u16)> = input
        .lines()
        .map(|line| {
            (
                line.split_whitespace().nth(0).unwrap(),
                line.split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u16>()
                    .unwrap(),
            )
        })
        .collect();
    lines.sort_by(|(this, _), (other, _)| {
        let this = *this;
        let other = *other;
        let cards = vec![
            "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
        ];
        let mut a = cards
            .iter()
            .map(|card| this.matches(card).count())
            .collect::<Vec<usize>>();
        a.sort();
        a.reverse();
        let mut b = cards
            .iter()
            .map(|card| other.matches(card).count())
            .collect::<Vec<usize>>();
        b.sort();
        b.reverse();
        let hands = vec![(5, 0), (4, 0), (3, 2), (3, 0), (2, 2), (2, 0), (1, 0)];
        let mut afound = false;
        let mut bfound = false;
        for (first, second) in hands {
            if a[0] >= first && a[1] >= second {
                afound = true;
            }
            if b[0] >= first && b[1] >= second {
                bfound = true;
            }

            if afound || bfound {
                break;
            }
        }
        if afound && bfound {
            for i in 0..5 {
                let a = this.chars().nth(i).unwrap();
                let b = other.chars().nth(i).unwrap();
                if a == b {
                    continue;
                }
                for card in cards.clone() {
                    if card.chars().nth(0).unwrap() == a {
                        return Ordering::Less;
                    } else {
                        if card.chars().nth(0).unwrap() == b {
                            return Ordering::Greater;
                        }
                    }
                }
            }
            unimplemented!();
        }
        return if afound {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    });
    lines.reverse();
    let ans = lines.iter().enumerate().fold(0u32, |sum, curr| {
        println!("{}", curr.1 .0);
        let (index, curr) = ((curr.0 + 1) as u32, (*curr.1).1 as u32);
        // println!("{index}, {curr}");
        sum + index * curr
    });
    println!("{ans}");
}
