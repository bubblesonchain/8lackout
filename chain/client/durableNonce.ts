// Pre-sign offline with a durable nonce; settle later. PLACEHOLDER.
export function makeDurableTx(needId: string) {
  return { needId, nonce: "NONCE-" + Math.random().toString(16).slice(2, 10), signed: true };
}
