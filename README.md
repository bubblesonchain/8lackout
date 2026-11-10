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
- `dist/sim.wasm` is built by CI from the two directories above.

## Run the tests

```bash
cd sim && cargo test
```

Not affiliated with or endorsed by the Percolator author.
