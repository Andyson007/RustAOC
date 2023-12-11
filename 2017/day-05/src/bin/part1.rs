fn main() {
    let input = include_str!("../../input.txt");
    let mut directions = input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut index: i32 = 0;
    let mut count = 0;
    let len = input.lines().count();
    while index >= 0 && index < len as i32 {
        let indexcopy = index;
        index += directions[index as usize];
        let direction: &mut i32 = &mut directions[indexcopy as usize];
        if *direction >= 3 {
            *direction -= 1;
        } else {
            *direction += 1;
        }
        count += 1;
    }
    println!("{count}");
}
