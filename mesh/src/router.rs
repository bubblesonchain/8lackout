// Store-and-forward, TTL, hop limits. PLACEHOLDER.
pub const MAX_HOPS: u8 = 8;
pub fn forward(hops: u8) -> bool { hops < MAX_HOPS }
