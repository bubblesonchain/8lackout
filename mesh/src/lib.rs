// Mesh transport + routing layer. PLACEHOLDER module graph.
pub mod lora;
pub mod ble;
pub mod webrtc;
pub mod gossip;
pub mod router;
pub mod packet;

pub fn version() -> &'static str { "mesh 0.1.0 (stub)" }
