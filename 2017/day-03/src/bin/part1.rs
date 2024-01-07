fn main() {
    // for input in 2.. {
    let input = 368078;
    {
        let ring = ((input as f64 - 1.0).sqrt().floor() as usize - 1) / 2;
        let ring = ring + 1;
        let last_in_ring = (ring * 2 + 1) * (ring * 2 + 1);
        let dist = (last_in_ring - input) % (2 * ring);
        println!("{input}: {}", dist.abs_diff(ring) + ring);
    }
}
