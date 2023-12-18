fn main() {
    let len = 35651584;
    let mut curr = "10111100110001111".chars().map(|c| c == '1').collect::<Vec<bool>>();
    while curr.len() < len {
        let mut next = curr.clone();
        next.push(false);
        for b in curr.iter().rev().map(|b| !b) {
            next.push(b);
        }
        curr = next;
    }
    curr = curr.iter().take(len).map(|x| *x).collect::<Vec<bool>>();
    while curr.len() % 2 == 0 {
        curr = curr
            .chunks(2)
            .map(|arr| arr[0] == arr[1])
            .collect::<Vec<bool>>();
    }
    for b in curr {
      print!("{}", if b {1} else {0});
    }
}
