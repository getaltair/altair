pub mod auth;
pub mod body;
pub mod objects;
pub mod read;
pub mod service;
pub mod store;
pub mod write;

#[cfg(feature = "testing")]
pub mod testing;

pub fn run() -> anyhow::Result<()> {
    Ok(())
}
