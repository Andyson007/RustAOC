use std::str::FromStr;

use day_10::asteroids::AsteroidField;

fn main() {
    let asteroid_field = AsteroidField::from_str(include_str!("../../input")).unwrap();
    let (ax, ay) = asteroid_field.evaluate_all().unwrap().0;
    let mut all = asteroid_field
        .get_all_asteroids(ax, ay)
        .into_iter()
        .map(|x| (x.0, (x.1, 0)))
        .collect::<Vec<_>>();
    let mut countdown = 200;

    let (dir, nth) = 'outer: loop {
        for (k, v) in all.iter_mut() {
            if v.0 == 0 {
                continue;
            }
            v.0 -= 1;
            v.1 += 1;
            countdown -= 1;
            if countdown == 0 {
                break 'outer (*k, v.1);
            }
        }
    };

    let ans = asteroid_field.get_nth_with_dir((ax, ay), dir, nth);

    println!("{}", ans.0 * 100 + ans.1);
}
