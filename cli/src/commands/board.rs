// List live needs on the mesh. PLACEHOLDER random data.
pub async fn run() -> anyhow::Result<()> {
    for (id, who, what) in [
        ("ND-1A2F", "ward-7", "insulin x2, bldg 12"),
        ("ND-0C91", "relay-3", "water, 40L, shelter B"),
        ("ND-77E0", "anon", "evac route needed, 6 people"),
    ] { println!("{id}  {who:<8}  {what}"); }
    Ok(())
}
