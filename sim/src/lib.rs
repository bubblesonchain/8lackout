//! Thin simulation wrapper around the unmodified Percolator v16 risk engine.
//! The wrapper only does what the engine README says a wrapper must do:
//! hold accounts, feed prices, and move (paper) tokens. All risk decisions
//! (margin, funding, liquidation, loss handling) come from the engine.

use percolator::{
    EngineAssetSlotV16Account, LiquidationRequestV16, Market, MarketGroupV16HeaderAccount,
    MarketGroupV16ViewMut, PortfolioAccountV16Account, PortfolioV16ViewMut, ProvenanceHeaderV16,
    ProvenanceHeaderV16Account, TradeRequestV16, V16Config, V16Error, POS_SCALE,
};
use std::cell::RefCell;
use std::fmt::Write as _;

pub const ASSETS: usize = 6;
pub const ACCOUNTS: usize = 8;
const MARKET_ID: [u8; 32] = [7; 32];
const OWNER: [u8; 32] = [9; 32];

pub struct Sim {
    header: MarketGroupV16HeaderAccount,
    markets: Vec<Market<u64>>,
    accounts: Vec<PortfolioAccountV16Account>,
    prices: [u64; ASSETS],
    slot: u64,
    deposited: u128,
    withdrawn: u128,
    pub last_error: &'static str,
}

fn err_name(e: V16Error) -> &'static str {
    match e {
        V16Error::InvalidConfig => "InvalidConfig",
        V16Error::ArithmeticOverflow => "ArithmeticOverflow",
        V16Error::ProvenanceMismatch => "ProvenanceMismatch",
        V16Error::HiddenLeg => "HiddenLeg",
        V16Error::InvalidLeg => "InvalidLeg",
        V16Error::Stale => "Stale",
        V16Error::BStale => "BStale",
        V16Error::LockActive => "LockActive",
        V16Error::NonProgress => "NonProgress",
        V16Error::RecoveryRequired => "RecoveryRequired",
        V16Error::CounterOverflow => "CounterOverflow",
        V16Error::CounterUnderflow => "CounterUnderflow",
    }
}

impl Sim {
    pub fn new(
        init_prices: [u64; ASSETS],
        initial_margin_bps: u64,
        maintenance_margin_bps: u64,
        liquidation_fee_bps: u64,
        max_move_bps: u64,
    ) -> Result<Sim, V16Error> {
        let mut cfg = V16Config::public_user_fund_with_market_slots(ASSETS as u16, ASSETS as u32, 0, 10);
        cfg.initial_margin_bps = initial_margin_bps;
        cfg.maintenance_margin_bps = maintenance_margin_bps;
        cfg.liquidation_fee_bps = liquidation_fee_bps;
        cfg.liquidation_fee_cap = 1_000_000_000_000;
        cfg.max_trading_fee_bps = 100;
        cfg.min_nonzero_mm_req = 100;
        cfg.min_nonzero_im_req = 101;
        cfg.max_price_move_bps_per_slot = max_move_bps;
        let mut header = MarketGroupV16HeaderAccount::new_dynamic(MARKET_ID, cfg, ASSETS as u32, 0)?;
        let mut markets: Vec<Market<u64>> = (0..ASSETS)
            .map(|i| Market::new(i as u64, EngineAssetSlotV16Account::default()))
            .collect();
        for i in 0..ASSETS {
            header.activate_empty_asset_slot_not_atomic(
                i as u32,
                &mut markets[i].engine,
                init_prices[i],
                (i + 1) as u64,
            )?;
        }
        let mut accounts = Vec::with_capacity(ACCOUNTS);
        for a in 0..ACCOUNTS {
            let ph = ProvenanceHeaderV16Account::from_runtime(&ProvenanceHeaderV16::new(
                MARKET_ID,
                [a as u8 + 1; 32],
                OWNER,
            ));
            let mut acct = PortfolioAccountV16Account::default();
            acct.init_empty_in_place(ph)?;
            accounts.push(acct);
        }
        let sim = Sim { header, markets, accounts, prices: init_prices, slot: 0, deposited: 0, withdrawn: 0, last_error: "" };
        Ok(sim)
    }

    /// Engine calls are documented as non-atomic: an `Err` must be rolled back by the
    /// caller, the way the SVM rolls back a failed transaction. We snapshot and restore.
    fn tx<R>(&mut self, f: impl FnOnce(&mut Sim) -> Result<R, V16Error>) -> Option<R> {
        let (h, m, a) = (self.header, self.markets.clone(), self.accounts.clone());
        let (p, s, d, w) = (self.prices, self.slot, self.deposited, self.withdrawn);
        match f(self) {
            Ok(r) => {
                self.last_error = "";
                Some(r)
            }
            Err(e) => {
                self.header = h;
                self.markets = m;
                self.accounts = a;
                self.prices = p;
                self.slot = s;
                self.deposited = d;
                self.withdrawn = w;
                self.last_error = err_name(e);
                None
            }
        }
    }

    pub fn deposit(&mut self, acct: usize, amount: u128) -> bool {
        if acct >= ACCOUNTS { return false; }
        self.tx(|s| {
            let mut market = MarketGroupV16ViewMut::new(&mut s.header, &mut s.markets);
            let mut p = PortfolioV16ViewMut::new(&mut s.accounts[acct]);
            market.deposit_not_atomic(&mut p, amount)?;
            s.deposited += amount;
            Ok(())
        })
        .is_some()
    }

    pub fn withdraw(&mut self, acct: usize, amount: u128) -> bool {
        if acct >= ACCOUNTS { return false; }
        self.tx(|s| {
            let mut market = MarketGroupV16ViewMut::new(&mut s.header, &mut s.markets);
            let mut p = PortfolioV16ViewMut::new(&mut s.accounts[acct]);
            market.full_account_refresh_not_atomic(&mut p)?;
            market.withdraw_not_atomic(&mut p, amount)?;
            s.withdrawn += amount;
            Ok(())
        })
        .is_some()
    }

    /// `size_q` > 0: account `a` buys from account `b` at the current oracle price.
    pub fn trade(&mut self, a: usize, b: usize, asset: usize, size_q: i128, fee_bps: u64) -> bool {
        if a >= ACCOUNTS || b >= ACCOUNTS || a == b || asset >= ASSETS { return false; }
        self.tx(|s| {
            let price = s.prices[asset];
            let (lo, hi) = if a < b { (a, b) } else { (b, a) };
            let (left, right) = s.accounts.split_at_mut(hi);
            let (ra, rb) = if a < b { (&mut left[lo], &mut right[0]) } else { (&mut right[0], &mut left[lo]) };
            let mut market = MarketGroupV16ViewMut::new(&mut s.header, &mut s.markets);
            let mut pa = PortfolioV16ViewMut::new(ra);
            let mut pb = PortfolioV16ViewMut::new(rb);
            market.full_account_refresh_not_atomic(&mut pa)?;
            market.full_account_refresh_not_atomic(&mut pb)?;
            market.execute_trade_with_fee_loss_stale_scoped_not_atomic(
                &mut pa,
                &mut pb,
                TradeRequestV16 { asset_index: asset, size_q, exec_price: price, fee_bps },
            )?;
            Ok(())
        })
        .is_some()
    }

    /// Advance the clock one slot and move one asset's oracle price (others keep theirs).
    /// The engine enforces its own per-slot price-move limit; a refused move leaves state unchanged.
    pub fn set_price(&mut self, asset: usize, price: u64, funding_rate_e9: i128) -> bool {
        if asset >= ASSETS { return false; }
        self.tx(|s| {
            let mut prices = s.prices;
            prices[asset] = price;
            let mut market = MarketGroupV16ViewMut::new(&mut s.header, &mut s.markets);
            let mut next = market.header.current_slot.get();
            for i in 0..ASSETS {
                next = next.max(market.markets[i].engine.asset.slot_last.get());
            }
            next += 1;
            for i in 0..ASSETS {
                let f = if i == asset { funding_rate_e9 } else { 0 };
                market.accrue_asset_to_not_atomic(i, next, prices[i], f, true)?;
            }
            market.set_asset_raw_oracle_target_not_atomic(asset, price)?;
            s.slot = next;
            s.prices = prices;
            Ok(())
        })
        .is_some()
    }

    pub fn liquidate(&mut self, acct: usize, asset: usize) -> Option<(u128, u128, u128)> {
        if acct >= ACCOUNTS || asset >= ASSETS { return None; }
        self.tx(|s| {
            let mut market = MarketGroupV16ViewMut::new(&mut s.header, &mut s.markets);
            let mut p = PortfolioV16ViewMut::new(&mut s.accounts[acct]);
            let o = market.liquidate_account_not_atomic(&mut p, LiquidationRequestV16 { asset_index: asset })?;
            Ok((o.closed_q, o.fee_charged, o.insurance_used))
        })
    }

    pub fn convert_pnl(&mut self, acct: usize) -> bool {
        if acct >= ACCOUNTS { return false; }
        self.tx(|s| {
            let mut market = MarketGroupV16ViewMut::new(&mut s.header, &mut s.markets);
            let mut p = PortfolioV16ViewMut::new(&mut s.accounts[acct]);
            market.full_account_refresh_not_atomic(&mut p)?;
            market.convert_released_pnl_to_capital_not_atomic(&mut p)?;
            Ok(())
        })
        .is_some()
    }

    /// The solvency checks the playground scores against.
    /// Returns (tokens_conserved, vault_covers_capital_and_insurance, engine_shape_valid).
    pub fn invariants(&mut self) -> (bool, bool, bool) {
        let vault = self.header.vault.get();
        let conserved = vault == self.deposited - self.withdrawn;
        let cap: u128 = self.accounts.iter().map(|a| a.capital.get()).sum();
        let covered = vault >= cap + self.header.insurance.get();
        let market = MarketGroupV16ViewMut::new(&mut self.header, &mut self.markets);
        let shape = market.validate_shape().is_ok();
        (conserved, covered, shape)
    }

    pub fn state_json(&mut self) -> String {
        let (c, v, sh) = self.invariants();
        let mut o = String::with_capacity(4096);
        let _ = write!(
            o,
            "{{\"slot\":{},\"vault\":\"{}\",\"insurance\":\"{}\",\"c_tot\":\"{}\",\"deposited\":\"{}\",\"withdrawn\":\"{}\",\"inv\":{{\"conserved\":{},\"covered\":{},\"shape\":{}}},\"err\":\"{}\",\"assets\":[",
            self.slot, self.header.vault.get(), self.header.insurance.get(), self.header.c_tot.get(),
            self.deposited, self.withdrawn, c, v, sh, self.last_error
        );
        for i in 0..ASSETS {
            let (l, s) = match self.markets[i].engine.asset.try_to_runtime() {
                Ok(a) => (a.oi_eff_long_q, a.oi_eff_short_q),
                Err(_) => (0, 0),
            };
            let _ = write!(o, "{}{{\"price\":{},\"oi_long\":\"{}\",\"oi_short\":\"{}\"}}", if i > 0 { "," } else { "" }, self.prices[i], l, s);
        }
        o.push_str("],\"accounts\":[");
        for a in 0..ACCOUNTS {
            // Health is read on a scratch copy so viewing never mutates engine state.
            let (mut h, mut m, mut acc) = (self.header, self.markets.clone(), self.accounts[a]);
            let cert = {
                let mut market = MarketGroupV16ViewMut::new(&mut h, &mut m);
                let mut p = PortfolioV16ViewMut::new(&mut acc);
                market.full_account_refresh_not_atomic(&mut p)
            };
            let (eq, im, mm, ok) = match cert {
                Ok(c) => (c.certified_equity, c.certified_initial_req, c.certified_maintenance_req, true),
                Err(_) => (0, 0, 0, false),
            };
            let _ = write!(
                o,
                "{}{{\"capital\":\"{}\",\"pnl\":\"{}\",\"equity\":\"{}\",\"im\":\"{}\",\"mm\":\"{}\",\"cert\":{},\"pos\":[",
                if a > 0 { "," } else { "" }, self.accounts[a].capital.get(), self.accounts[a].pnl.get(), eq, im, mm, ok
            );
            let mut pos = [0i128; ASSETS];
            for leg in self.accounts[a].legs.iter() {
                if let Ok(l) = leg.try_to_runtime() {
                    if l.active && (l.asset_index as usize) < ASSETS {
                        pos[l.asset_index as usize] = l.basis_pos_q;
                    }
                }
            }
            for i in 0..ASSETS {
                let _ = write!(o, "{}\"{}\"", if i > 0 { "," } else { "" }, pos[i]);
            }
            o.push_str("]}");
        }
        o.push_str("]}");
        o
    }
}

// ---------------- C ABI for the browser (wasm32-unknown-unknown, no bindgen) ----------------

thread_local! {
    static SIM: RefCell<Option<Sim>> = const { RefCell::new(None) };
    static OUT: RefCell<String> = const { RefCell::new(String::new()) };
}

fn with<R>(d: R, f: impl FnOnce(&mut Sim) -> R) -> R {
    SIM.with(|s| match s.borrow_mut().as_mut() { Some(sim) => f(sim), None => d })
}
fn u(hi: u64, lo: u64) -> u128 { ((hi as u128) << 64) | lo as u128 }

#[no_mangle]
pub extern "C" fn sim_init(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, im_bps: u64, mm_bps: u64, liq_bps: u64, max_move_bps: u64) -> i32 {
    match Sim::new([p0, p1, p2, p3, p4, p5], im_bps, mm_bps, liq_bps, max_move_bps) {
        Ok(s) => { SIM.with(|c| *c.borrow_mut() = Some(s)); 1 }
        Err(_) => 0,
    }
}
#[no_mangle]
pub extern "C" fn sim_deposit(acct: u32, amount: u64) -> i32 { with(0, |s| s.deposit(acct as usize, amount as u128) as i32) }
#[no_mangle]
pub extern "C" fn sim_withdraw(acct: u32, amount: u64) -> i32 { with(0, |s| s.withdraw(acct as usize, amount as u128) as i32) }
#[no_mangle]
pub extern "C" fn sim_trade(a: u32, b: u32, asset: u32, size_q: i64, fee_bps: u64) -> i32 { with(0, |s| s.trade(a as usize, b as usize, asset as usize, size_q as i128, fee_bps) as i32) }
#[no_mangle]
pub extern "C" fn sim_set_price(asset: u32, price: u64, funding_e9: i64) -> i32 { with(0, |s| s.set_price(asset as usize, price, funding_e9 as i128) as i32) }
#[no_mangle]
pub extern "C" fn sim_liquidate(acct: u32, asset: u32) -> i32 { with(0, |s| s.liquidate(acct as usize, asset as usize).is_some() as i32) }
#[no_mangle]
pub extern "C" fn sim_convert(acct: u32) -> i32 { with(0, |s| s.convert_pnl(acct as usize) as i32) }
#[no_mangle]
pub extern "C" fn sim_state() -> *const u8 {
    let json = with(String::from("{}"), |s| s.state_json());
    OUT.with(|o| { *o.borrow_mut() = json; o.borrow().as_ptr() })
}
#[no_mangle]
pub extern "C" fn sim_state_len() -> u32 { OUT.with(|o| o.borrow().len() as u32) }
#[allow(dead_code)]
fn _unused() { let _ = (u(0, 0), POS_SCALE); }
