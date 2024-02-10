fn main() {
    let a: i64 = 0;
    let mut b: i64;
    let mut c: i64;
    let mut d: i64;
    let mut e: i64;
    let mut f: i64;
    let mut g: i64;
    let mut h = 0i64;
    let mut count = 0;
    b = 57; // set b 57
    c = b; // set c b
    if a != 0 {
        b *= 100; // mul b 100
        count += 1;
        b -= -100000; // sub b -100000
        c = b; // set c b
        c -= 17000; // sub c -17000
    }
    loop {
        f = 1; // set f 1
        d = 2; // set d 2
        loop {
            e = 2; // set e 2
            loop {
                g = d; // set g d
                g *= e; // mul g e
                count += 1;
                g -= b; // sub g b
                if g == 0 {
                    f = 0; // set f 0
                }
                e += 1; // sub e -1
                g = e; // set g e
                g -= b; // sub g b
                if g == 0 {
                    break;
                }
            }
            d += 1; // sub d -1
            g = d; // set g d
            g -= b; // sub g b
            if g == 0 {
                break;
            }
        }
        if f == 0 {
            h += 1; // sub h -1
        }
        g = b; // set g b
        g -= c; // sub g c
        if g == 0 {
            println!("{h}");
            println!("{count}");
            return;
        }
        b += 17; // sub b -17
    }
}

// set b 57
// set c b
// jnz a 2
// jnz 1 5
// mul b 100
// sub b -100000
// set c b
// sub c -17000
// set f 1
// set d 2
// set e 2
// set g d
// mul g e
// sub g b
// jnz g 2
// set f 0
// sub e -1
// set g e
// sub g b
// jnz g -8
// sub d -1
// set g d
// sub g b
// jnz g -13
// jnz f 2
// sub h -1
// set g b
// sub g c
// jnz g 2
// jnz 1 3
// sub b -17
// jnz 1 -23
