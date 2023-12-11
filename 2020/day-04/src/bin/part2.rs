use std::collections::HashMap;

fn main() {
    let ans = include_str!("../../example.txt")
        .split("\r\n\r\n")
        .map(|line| {
            line.split_whitespace()
                .map(|a| {
                    (
                        a.split(":").nth(0).unwrap().to_string().trim().to_string(),
                        a.split(":").nth(1).unwrap().to_string().trim().to_string(),
                    )
                })
                .collect::<HashMap<String, String>>()
        })
        .filter(|x| {
            if let Some(x) = x.get("byr") {
                let val = x.parse::<u16>().unwrap();
                if x.len() != 4 || (val < 1920 && val > 2002) {
                    return false;
                }
            } else {
                return false;
            }
            if let Some(x) = x.get("iyr") {
                let val = x.parse::<u16>().unwrap();
                if x.len() != 4 || (val < 2010 && val > 2020) {
                    return false;
                }
            } else {
                return false;
            }
            if let Some(x) = x.get("eyr") {
                let val = x.parse::<u16>().unwrap();
                if x.len() != 4 || (val < 2020 && val > 2030) {
                    return false;
                }
            } else {
                return false;
            }
            if let Some(x) = x.get("hgt") {
                if x.len()<=2 {
                  return false;
                }
                let val = x[0..x.len() - 2].parse::<u16>().unwrap();
                if &x[x.len() - 2..=x.len() - 1] == "cm" {
                    if val < 150 || val > 193 {
                        return false;
                    }
                } else if &x[x.len() - 2..=x.len() - 1] == "in" {
                    if val < 59 || val > 76 {
                        return false;
                    }
                } else {
                  return false;
                }
            } else {
              return false;
            }
            if let Some(x) = x.get("hcl") {
              if x.chars().nth(0).unwrap() != '#' {
                return false;
              }
              if x.len() != 7 {
                return false;
              }
              if !match u32::from_str_radix(&x[1..x.len()], 16) {
                Ok(_) => true,
                Err(_) => false
              } {
                return false;
              }
            } else {
              return false;
            }
            if let Some(x) = x.get("ecl") {
              if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x.as_str()) {
                return false;
              }
            }
            if let Some(x) = x.get("pid") {
              if x.len()!=9 {
                return false;
              }
              if !match x.parse::<u32>() {
                Ok(_) => true,
                Err(_) => false
              } {
                return false;
              }
            } else {
              return false;
            }
            true
        })
        .count();
    println!("{ans}")
}
