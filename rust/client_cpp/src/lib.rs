mod asynchronous;
mod synchronous;

use client::SyncGridClient;

/// A wrapper to avoid `cxx`'s limitation that types need to be implemented in this crate.
struct SyncGridClientWrapper(SyncGridClient);

///
fn connect_sync_grid_client(
    host_name: &str,
    port: u32,
    client_name: String,
) -> Result<Box<SyncGridClientWrapper>, Box<dyn std::error::Error>> {
    // Wrap the `SyncGridClient` in a `SyncGridClientWrapper`.
    Ok(Box::new(SyncGridClientWrapper(
        client::connect_sync_grid_client(host_name, port, client_name)?,
    )))
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type SyncGridClientWrapper;

        pub fn connect_sync_grid_client(
            host_name: &str,
            port: u32,
            client_name: String,
        ) -> Result<Box<SyncGridClientWrapper>>;
    }
}
