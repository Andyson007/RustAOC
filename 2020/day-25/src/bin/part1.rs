fn main() {
    let door = 14012298;
    let card = 74241;
    // let door = 17807724;
    // let card = 5764801;
    let subject: u64 = 7;
    let mut value: u64 = 1;
    let mut cardcount = 0;
    loop {
        cardcount += 1;
        value *= subject;
        value %= 20201227;
        if value == card {
            break;
        }
    }
    value = 1;
    loop {
        value *= subject;
        value %= 20201227;
        if value == door {
            break;
        }
    }
    let subject = value;
    value = 1;
    for _ in 0..cardcount {
        value *= subject;
        value %= 20201227;
    }
    println!("{value}");
}
