# 8LACKOUT protocol (draft)

Frames: {id, kind: need|claim|proof|timeline, hops, payload, sig}.
Offline: sign + durable-nonce locally, gossip over mesh, dedupe by id, TTL by hops.
Online: a beacon batches the queue → proof-of-aid + receipts + timeline on Solana.
