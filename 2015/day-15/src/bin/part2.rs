use std::cmp;
use std::iter::repeat;

#[derive(Clone)]
struct Thing<'a> {
    name: &'a str,
    ingredients: Vec<i32>,
}

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let name = line.split(":").nth(0).unwrap();
            let values = line
                .split(":")
                .skip(1)
                .collect::<String>()
                .split(",")
                .map(|x| x.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Thing {
                name,
                ingredients: values,
            }
        })
        .collect::<Vec<Thing>>();
    let mut amounts = repeat(0).take(input.len()).collect::<Vec<i32>>();
    let mut max = 0;
    while amounts.iter().any(|x| *x != 100) {
        for i in (0..input.len()).rev() {
            if amounts[i] == 100 {
                amounts[i] = 0;
            } else {
                amounts[i] += 1;
                break;
            }
        }
        if amounts.iter().sum::<i32>() != 100 {
            continue;
        }
        let mut recepie = repeat(0).take(5).collect::<Vec<i32>>();
        let calories = amounts
            .iter()
            .zip(input.clone())
            .map(|(a, b)| a * b.ingredients[4])
            .sum::<i32>();
        if calories!=500 {
          continue;
        }
        for (amount, ingredient) in amounts.iter().zip(input.clone()) {
            recepie = recepie
                .iter()
                .zip(ingredient.ingredients)
                .take(4)
                .map(|(a, b)| a + b * *amount)
                .collect::<Vec<i32>>();
        }
        let value = recepie.iter().fold(1, |sum, curr| {
            if *curr < 0 {
                return 0;
            }
            sum * *curr
        });
        max = cmp::max(value, max);
    }
    println!("{max}");
}
