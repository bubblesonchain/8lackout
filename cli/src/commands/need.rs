// Post a need; the on-device agent structures it. PLACEHOLDER.
pub async fn run(text: &str) -> anyhow::Result<()> {
    let id = format!("ND-{:04X}", (text.len() * 7919) % 0xFFFF);
    println!("[8lackout] need {id} structured by operator → gossiped to mesh");
    println!("           raw: {text}");
    Ok(())
}
