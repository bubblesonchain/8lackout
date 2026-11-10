# Ghost Perps playground

Paper perpetuals running on the unmodified [Percolator](https://github.com/aeyakovenko/percolator)
risk engine by Anatoly Yakovenko, compiled to WebAssembly so it runs in a browser tab.

**Paper money only. Not audited. Nothing here holds or moves real funds.**
The engine's own README says: "Educational research project. Not production ready.
Not audited. Do not use with real funds." That applies to this repo too.

## Layout

- `engine/` is Percolator at commit `37cb14ee1436eaf3b7d97e1901b0dc1bf38f5311`, byte for byte.
  CI checks it against that commit's tree. Apache-2.0, see `NOTICE`.
- `sim/` is a thin wrapper. It holds the accounts, feeds prices and moves paper tokens.
  Every risk decision (margin, liquidation, fees, loss handling) is made by the engine.
- `web/index.src.html` is the terminal page. `__WASM__` is replaced with the base64 of the build.
- `dist/sim.wasm` is built by CI from the two directories above.

## Status

Work in progress. Deposits, trades, the 10x margin check, partial liquidation, fees to
insurance and the three solvency checks all work, and the solvency checks hold across the
soak test. Known gap: after a liquidation the engine scales the opposite side down and
blocks new risk in that market until that side has drained. With a single house account on
the other side of every trade the market stays locked, so most orders are refused after the
first liquidations. The wrapper's market setup needs to change; the engine is behaving as
specified.

There is no official token.

## Run the tests

```bash
cd sim && cargo test
```

Not affiliated with or endorsed by the Percolator author.
