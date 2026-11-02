# one-command dev tasks
up name="dev-node":
    cargo run -p cli -- up --name {{name}}
beacon:
    cargo run -p beacon -- --solar --uplink auto
app:
    pnpm --filter ./app dev
slice:
    ./scripts/devnet-slice.sh
