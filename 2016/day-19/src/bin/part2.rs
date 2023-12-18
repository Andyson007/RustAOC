fn main() {
    let mut elves: Vec<usize> = (1..=3012210).collect::<Vec<usize>>();
    let mut index = 0;
    while elves.len() != 1 {
        if elves.len() %10000 == 0{
          println!("{}", elves.len());
        }
        index = (index + 1) % elves.len();
        elves.remove((index + elves.len() / 2 - 1) % elves.len());
        index = index % elves.len();
    }//doesn't work for whatever reason
    print!("{elves:?}");
}
