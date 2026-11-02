<div align="center">

# 8LACKOUT

### Keep a city coordinating when the lights go out.

An offline-first resilience network for blackouts, disasters, and war zones. Phones form their own radio mesh, an on-device Claude runs the coordination, and every action is proven on Solana the moment any signal returns.

[![mesh](https://img.shields.io/badge/network-LoRa%20%2B%20BLE%20mesh-111)](#) [![ai](https://img.shields.io/badge/AI-on--device%20Claude-111)](#) [![chain](https://img.shields.io/badge/ledger-Solana%20(offline)-111)](#) [![license](https://img.shields.io/badge/license-MIT-111)](#)

</div>

---

## What it is

When the grid, the towers, or the internet go down, coordination dies first. 8LACKOUT assumes they're already off. It turns a pile of ordinary phones and a few cheap radios into a working command layer: people find each other, needs get matched to responders, aid is proven, and the record of what happened survives — all with no internet, no cell, no cloud.

> Civilian infrastructure only. No targeting, no surveillance. It keeps people findable, aid accountable, and the truth timestamped when every normal system is down.

---

## The tool

`8lackout` is a single binary that turns any device into a node. Same command on a phone, a laptop, or a solar beacon — it finds whatever radios are present and joins the mesh.

```bash
# install
curl -fsSL https://get.8lackout.net | sh

# bring a node online (auto-detects LoRa / BLE / any uplink)
8lackout up --name "ward-7-clinic"

# post a need — the on-device operator structures and localizes it
8lackout need "insulin, 2 people, Khoroolol 4, building 12"

# see what's live on the mesh around you
8lackout board

# claim and deliver, co-signing proof on handoff
8lackout claim <id>
8lackout deliver <id> --with <responder>
```

Most people never touch the CLI — they open the **app** (an offline PWA: map, needs board, QR handoff, scanner). The CLI is what runs the beacons and lets power users script the network.

---

## How it works

- **Mesh, no towers.** Nodes talk over LoRa radio and Bluetooth, relaying for each other so a message hops across a neighborhood with zero infrastructure. Any real uplink anywhere (one satellite phone, one bar of signal) heals the whole mesh.
- **The operator runs offline.** A quantized Claude model lives on the device. It triages needs, translates at checkpoints, and turns a panicked voice note into a structured, routable request — with no connection. When a link returns, the full Claude verifies and co-signs the record.
- **Proof that can't be faked.** Every delivery is co-signed by both sides as a proof-of-aid record using Solana durable nonces — valid while fully offline. Donors and coordinators later see exactly what reached whom.
- **A record nobody can erase.** Event hashes, evidence, and the blackout timeline commit to an immutable ledger, so the account of what happened outlives the blackout.

| Layer | Normal assumption | 8LACKOUT |
|---|---|---|
| **Network** | Towers + ISP | LoRa + BLE mesh, phone-to-phone relay |
| **AI** | Cloud API | On-device Claude, offline; full Claude co-signs on reconnect |
| **Trust** | Bank / online RPC | Solana txns pre-signed with durable nonces, gossiped over mesh, settled on reconnect |
| **Scale** | One row per event | State compression → millions of aid receipts for pennies |
| **Power** | The grid | Solar beacons as always-on anchor relays |

The whole trick: **nothing needs connectivity at the moment of action.** You sign, log, and coordinate locally; the network reconciles later, and the chain is the referee that makes the offline record trustworthy after the fact.

---

## Architecture

```
          ┌──────────── OFFLINE (the blackout) ────────────┐        ┌─ ONLINE (signal returns) ─┐
          │                                                 │        │                            │
  voice / ┌─────────┐   LoRa/BLE mesh   ┌─────────┐   durable-nonce  ┌──────────┐   settle + verify ┌──────────┐
  text ──▶│ app PWA │◀────gossip───────▶│ app PWA │──signed txns────▶│  beacon  │──────batch───────▶│  Solana  │
          │ + agent │                   │ + agent │                  │ (solar)  │                   │ programs │
          └────┬────┘                   └────┬────┘                  └────┬─────┘                   └────┬─────┘
               │ on-device Claude            │ relay for peers            │ store-and-forward           │ proof-of-aid
               ▼ triage / translate          ▼                            ▼ holds queue until uplink    ▼ receipts + timeline
```

```
8lackout/
├── cli/         # the `8lackout` binary — node, needs, claim, deliver, beacon
├── mesh/        # radio firmware + relay daemon (LoRa, BLE, WebRTC bridge)
├── agent/       # on-device Claude: triage, translation, voice→structured need
├── chain/       # Solana programs: proof-of-aid, durable-nonce queue, receipts, timeline
├── app/         # offline-first PWA (map, needs board, QR handoff, scanner)
├── beacon/      # solar anchor node (store-and-forward, uplink reconciler)
└── docs/        # threat model, field guide, protocol spec
```

### The three flows

1. **Need → match.** Someone posts a need; the on-device agent structures and localizes it; it's gossiped across the mesh; a nearby responder claims it.
2. **Deliver → prove.** On handoff, both phones co-sign a proof-of-aid record (durable nonce, fully offline). It rides the mesh toward a beacon.
3. **Reconnect → settle.** When a beacon gets any uplink, it batches the queue to Solana: receipts commit, the day's timeline is co-signed by the full Claude, and coordinators see verified aid.

---

## Run a beacon

A beacon is any always-on node with power and an antenna — a Raspberry Pi with a LoRa hat and a solar panel is enough. It stores and forwards the mesh when phones go dark and reconciles everything to chain when it gets a link.

```bash
8lackout beacon --solar --uplink auto
```

---

## Principles

- Civilians first. No targeting, no surveillance, no offensive use.
- Offline is the default; online is the exception.
- The chain exists to make an offline record trustworthy — not to require connectivity.
- Open source, auditable, forkable. MIT.
