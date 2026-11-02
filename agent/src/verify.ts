// Full Claude co-sign when a link returns. PLACEHOLDER.
export async function cosign(recordHash: string): Promise<{ ok: boolean; sig: string }> {
  return { ok: true, sig: "COSIGN-" + recordHash.slice(0, 8) };
}
