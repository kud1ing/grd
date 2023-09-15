pub mod grid_server_interface {
    // Include the generated rust module.
    tonic::include_proto!("grid_server_interface");
}

/// The grid client ID type.
pub type ClientId = u32;

/// The job ID type.
pub type JobId = u64;

/// The raw byte data.
pub type RawData = Vec<u8>;

/// The service ID type.
pub type ServiceId = u32;

/// The service version type.
pub type ServiceVersion = u32;

pub use grid_server_interface::grid_server_client::GridServerClient;
pub use grid_server_interface::grid_server_server::{GridServer, GridServerServer};
pub use grid_server_interface::*;
