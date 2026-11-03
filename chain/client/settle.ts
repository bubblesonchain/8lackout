// Batch the offline queue to Solana when a link returns. PLACEHOLDER.
export async function settleQueue(items: unknown[]) {
  return { settled: items.length, sig: "SETTLE-" + Math.random().toString(16).slice(2, 10) };
}
