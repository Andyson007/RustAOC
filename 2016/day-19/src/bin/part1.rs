fn main() {
    let mut elves: Vec<usize> = (1..=3012210).collect::<Vec<usize>>();
    let mut index = 1;
    while elves.len() != 1 {
      if elves.len() %10000 == 0{
        println!("{}", elves.len());
      }
        elves.remove(index);
        index = (index+1)%elves.len();
    }//somehow doesn't work
    print!("{elves:?}");
}
