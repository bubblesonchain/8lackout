use sim::*;
const U: u64 = 1_000_000; // 1 USD in atoms
const Q: i128 = 1_000_000; // 1 unit
#[test]
fn round_trip_and_liquidation() {
    let mut s = Sim::new([150*U, 100_000*U, 3_000*U, 140*U, 350*U, 230*U], 1_000, 500, 50, 400).expect("init");
    assert!(s.deposit(0, 1_000 * U as u128), "{}", s.last_error);
    assert!(s.deposit(7, 1_000_000 * U as u128), "{}", s.last_error);
    // 10x long: 50 units of asset 0 at $150 = $7,500 notional on $1,000
    assert!(s.trade(0, 7, 0, 50 * Q, 10), "trade: {}", s.last_error);
    assert!(!s.trade(0, 7, 0, 500 * Q, 10), "over-leverage must be refused");
    { let mut o = String::new(); s.write_state(&mut o); println!("{}", &o[..300]); }
    let mut p = 150 * U;
    for _ in 0..4 { p = p * 97 / 100; assert!(s.set_price(0, p, 0), "price: {}", s.last_error); }
    { let mut o = String::new(); s.write_state(&mut o); println!("{}", &o[..300]); }
    assert!(s.deposit(1, 5_000 * U as u128));
    assert!(s.trade(1, 7, 2, Q, 10), "trade after price moves: {}", s.last_error);
    let l = s.liquidate(0, 0);
    println!("liq {:?} {}", l, s.last_error);
    assert!(l.is_some());
    let (a, b, c) = s.invariants();
    assert!(a && b && c);
    { let mut o = String::new(); s.write_state(&mut o); println!("{}", &o[..300]); }
}
