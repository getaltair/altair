pub mod store;

#[cfg(feature = "testing")]
pub mod testing;

pub fn run() -> anyhow::Result<()> {
    Ok(())
}
