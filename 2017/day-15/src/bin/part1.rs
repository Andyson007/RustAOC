fn main() {
    let mut A: u64 = 618;
    let mut B: u64 = 814;
    let mut count = 0;
    for _ in 0..40000000 {
        A = (A*16807)%2147483647;
        B = (B*48271)%2147483647;
        if A & 0xffff == B & 0xffff {
          count+=1;
        }
    }
    println!("{count}");
}
