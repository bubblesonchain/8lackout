// Wire format: signed, chunked frames. PLACEHOLDER.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub id: String,
    pub kind: String,   // need | claim | proof | timeline
    pub hops: u8,
    pub payload: Vec<u8>,
    pub sig: String,
}
