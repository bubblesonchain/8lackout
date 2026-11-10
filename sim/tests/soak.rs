use sim::*;
const U: u64 = 1_000_000; const Q: i128 = 1_000_000;
#[test]
fn many_slots_stay_tradable_and_solvent() {
    let mut s = Sim::new([150*U, 100_000*U, 3_000*U, 140*U, 350*U, 230*U], 1_000, 500, 50, 400).unwrap();
    for a in 0..7 { assert!(s.deposit(a, 10_000 * U as u128)); }
    assert!(s.deposit(7, 5_000_000 * U as u128));
    let mut p = [150*U, 100_000*U, 3_000*U, 140*U, 350*U, 230*U];
    let mut x: u64 = 88172645463325252; let mut fills = 0; let mut refused = 0; let mut liqs = 0;
    for t in 0..600u64 {
        for i in 0..6 { x ^= x << 13; x ^= x >> 7; x ^= x << 17; let r = (x % 601) as i64 - 300; p[i] = (p[i] as i64 + p[i] as i64 * r / 10_000) as u64; }
        assert!(s.tick(p), "tick {} {}", t, s.last_error);
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        let a = (x % 7) as usize; let i = ((x >> 8) % 6) as usize; let dir = if (x >> 16) & 1 == 0 { 1 } else { -1 };
        let usd = 3_000 + ((x >> 20) % 60_000) as i128;
        let q = usd * 1_000_000 * 1_000_000 / p[i] as i128 * dir;
        if s.trade(a, 7, i, q, 10) { fills += 1 } else { refused += 1 }
        liqs += s.last_crank.count_ones();
        let (c, v, sh) = s.invariants();
        assert!(c && v && sh, "invariant broke at {}", t);
    }
    println!("fills {} refused {} liqs {}", fills, refused, liqs);
    assert!(fills > 0);
}
