// PLACEHOLDER test.
import { makeDurableTx } from "../client/durableNonce.ts";
const tx = makeDurableTx("ND-1A2F");
if (!tx.signed) throw new Error("expected signed durable tx");
console.log("ok: durable tx built offline", tx.nonce);
