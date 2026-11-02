// Message propagation with dedupe. PLACEHOLDER.
use std::collections::HashSet;
pub struct Gossip { seen: HashSet<String> }
impl Gossip {
    pub fn new() -> Self { Self { seen: HashSet::new() } }
    pub fn accept(&mut self, id: &str) -> bool { self.seen.insert(id.to_string()) }
}
