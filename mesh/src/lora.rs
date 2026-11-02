// LoRa transport. PLACEHOLDER. Real impl drives an SX126x radio.
pub struct Lora { pub freq_mhz: f32 }
impl Lora {
    pub fn open() -> Self { Self { freq_mhz: 868.0 } }
    pub fn send(&self, bytes: &[u8]) { let _ = bytes; /* TODO */ }
}
