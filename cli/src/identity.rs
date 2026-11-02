// Device identity placeholder: an ed25519 keypair = this node's handle.
pub struct Identity { pub handle: String, pub pubkey: String }

pub fn load_or_create() -> Identity {
    // TODO: persist a real keypair. Random stub for now.
    Identity { handle: "anon".into(), pubkey: "STUBPUBKEY1111111111111111111111111111111111".into() }
}
