// Solar anchor node: store-and-forward + reconcile to chain. PLACEHOLDER.
mod forward; mod uplink; mod power;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[beacon] {}", mesh::version());
    println!("[beacon] solar={} battery={}%", power::is_solar(), power::battery());
    forward::start();
    uplink::reconcile_loop().await;
    Ok(())
}
