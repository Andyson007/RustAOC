fn main() {
    code(0);
}

fn code(id: i64) {
    let mut i = 0;
    let mut a = 0;
    let mut b = 0;
    let mut f = 0;
    let mut p = id;

    // set i 31
    i = 31;
    // set a 1
    a = 1;
    // mul p 17
    p *= 17;
    // jgz p p
    if p != 17 {
        while {
            // mul a 2
            a *= 2;
            // add i -1
            i -= 1;
            // jgz i -2
            i > 0
        } {}
        // add a -1
        a -= 1;
        // set i 127
        i = 127;
        // set p 735
        p = 735;
        while {
            // mul p 8505
            p *= 8505;
            // mod p a
            p %= a;
            // mul p 129749
            p *= 129749;
            // add p 12345
            p += 12345;
            // mod p a
            p %= a;
            // set b p
            b = p;
            // mod b 10000
            b %= 10000;
            // snd b
            snd(b);
            // add i -1
            i -= 1;
            // jgz i -9
            i > 0
        } {}
    }

    // jgz a 3
    if a <= 0 {
        while {
            // rcv b
            b = rcv();
            // jgz b -1
            b > 0
        } {}
    }
    // set f 0
    f = 0;
    while {
        // set i 126
        i = 126;
        // rcv a
        a = rcv();
        while {
            // rcv b
            b = rcv();
            // set p a
            p = a;
            // mul p -1
            p *= -1;
            // add p b
            p += b;
            // jgz p 4
            if p > 0 {
                // snd b
                snd(b);
                // set f 1
                f = 1;
            } else {
                // snd a
                snd(a);
                // set a b
                a = b;
                // jgz 1 3 -> jmp 3
            }
            // add i -1
            i -= 1;
            // jgz i -11
            i > 0
        } {}
        // snd a
        snd(a);
        // jgz f -16
        f > 0
    } {}
    if a > 0 {
        loop {}
    }
    // jgz a -19
    {}
}

fn snd(data: i64) {
    println!("{data}");
}
fn rcv() -> i64 {
    todo!()
}
