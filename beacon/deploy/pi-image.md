# Raspberry Pi + LoRa hat beacon

1. Flash Raspberry Pi OS Lite.
2. Attach an SX126x LoRa hat and a solar/battery HAT.
3. `cargo build -p beacon --release` and copy the binary to `/usr/local/bin/beacon`.
4. `sudo cp deploy/8lackout-beacon.service /etc/systemd/system/ && sudo systemctl enable --now 8lackout-beacon`.
