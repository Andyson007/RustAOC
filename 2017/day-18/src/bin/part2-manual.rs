use std::{
    array,
    cmp::{max, min},
    iter,
};

fn code0() {
    let a = (1 << 31) - 1;
    let p = 735;
    for elem in generate_list(p, a).take(127) {
        snd(elem)
    }

    let mut should_stop = false;
    while {
        let mut a = rcv();
        for _ in 0..126 {
            let b = rcv();
            snd(max(a, b));
            a = min(a, b);
            if a < b {
                should_stop = true;
            }
        }
        snd(a);
        should_stop
    } {}
}

fn generate_list(mut p: i64, a: i64) -> impl Iterator<Item = i64> {
    iter::from_fn(move || {
        p *= 8505;
        p %= a;
        p *= 129749;
        p += 12345;
        p %= a;
        Some(p % 10000)
    })
}

fn code1() {
    while {
        let mut values: [_; 127] = array::from_fn(|_| rcv());
        let should_continue = bubble_iter(&mut values);
        for value in values {
            snd(value)
        }
        should_continue
    } {}
}

fn main() {
    let mut list = generate_list(735, (1 << 31) - 1).take(127).collect::<Vec<_>>();
    let mut count = 0;
    while {
        bubble_iter(&mut list);
        count += 1;
        bubble_iter(&mut list)
    } {}
    println!("{}", count * 127);
}

fn bubble_iter(data: &mut [i64]) -> bool {
    let mut did_swap = false;
    for i in 0..data.len() - 1 {
        if data[i] < data[i + 1] {
            data.swap(i, i + 1);
            did_swap = true;
        }
    }
    did_swap
}

fn snd(data: i64) {
    println!("{data}");
}
fn rcv() -> i64 {
    todo!()
}
