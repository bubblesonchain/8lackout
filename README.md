# Pppercolator

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

Working: deposits, trades, the 10x margin check, margin calls, fees to insurance, and three
solvency checks that hold across a 600-slot soak test with the market staying active.

Market setup (wrapper policy, not engine rules): each trader faces a dedicated house account.
When the engine certifies a trader below maintenance margin, the house closes the position
at the oracle price as an ordinary trade. If the engine refuses that, its own liquidation
runs in the keeper crank. An earlier setup with one shared house account locked markets
after every engine liquidation, because the engine blocks new risk on a side it has scaled
down until that side drains.

Prices, candles and funding rates: live from Hyperliquid's public info API (main perps for
crypto, the XYZ market for stock perps), with a random simulation as fallback. Not done: funding rates are zero and nothing is on chain.

$PPP on Pump.fun (`Ho7iohoM6UN96yoxzBNzz9aXZpNW3AyUVuEmBdAQpump`) was launched by the Pppercolator team.
It is a memecoin, gives no claim on this project, and is not connected to the Percolator author.

## Run the tests

```bash
cd sim && cargo test
```

Not affiliated with or endorsed by the Percolator author.
