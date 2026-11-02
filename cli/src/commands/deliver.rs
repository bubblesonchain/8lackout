// Co-sign proof-of-aid on handoff (offline, durable nonce). PLACEHOLDER.
pub async fn run(id: &str, with: Option<&str>) -> anyhow::Result<()> {
    let peer = with.unwrap_or("recipient");
    println!("[8lackout] proof-of-aid for {id} co-signed with {peer}");
    println!("           queued offline → settles to Solana on next uplink");
    Ok(())
}
