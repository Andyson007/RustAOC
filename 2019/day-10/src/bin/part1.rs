use std::str::FromStr;

use asteroids::AsteroidField;
use day_10::*;

fn main() {
    let asteroid_field = AsteroidField::from_str(include_str!("../../input")).unwrap();
    let amount = asteroid_field.evaluate_all().unwrap().1;
    println!("{amount}");
}
