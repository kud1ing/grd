pub mod server_interface {
    // Include the generated rust module.
    tonic::include_proto!("server_interface");
}

/// The client ID type.
pub type ClientId = u32;

/// The job ID type.
pub type JobId = u64;

/// The raw byte data.
pub type RawData = Vec<u8>;

/// The service ID type.
pub type ServiceId = u32;

/// The server version type.
pub type ServiceVersion = u32;

pub use crate::server_interface::{
    Job, JobFetchRequest, JobFetchResponse, JobSubmitRequest, JobSubmitResponse,
    RegisterClientRequest, RegisterClientResponse, Result, ResultFetchRequest, ResultFetchResponse,
    ResultSubmitRequest, ResultSubmitResponse,
};
pub use server_interface::grid_client::GridClient;
pub use server_interface::grid_server::{Grid, GridServer};
