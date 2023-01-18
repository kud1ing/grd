use crate::{connect_async_grid_client, AsyncGridClient};
use server_interface::{
    JobFetchResponse, JobSubmitResponse, ResultFetchResponse, ResultSubmitResponse, ServiceId,
    ServiceVersion,
};
use tokio::runtime::Runtime;
use tonic::{Response, Status};

///
pub struct SyncGridClient {
    async_grid_client: AsyncGridClient,
    async_runtime: Runtime,
}

///
pub fn connect_sync_grid_client(
    host_name: &str,
    port: u32,
) -> Result<SyncGridClient, Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // Connect the server_interface client.
    let asynchronous_grid_client = runtime.block_on(connect_async_grid_client(host_name, port))?;

    Ok(SyncGridClient {
        async_grid_client: asynchronous_grid_client,
        async_runtime: runtime,
    })
}

impl SyncGridClient {
    ///
    pub fn fetch_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
    ) -> Result<Response<JobFetchResponse>, Status> {
        self.async_runtime.block_on(
            self.async_grid_client
                .fetch_job(service_id, service_version),
        )
    }

    ///
    pub fn fetch_results(&mut self) -> Result<Response<ResultFetchResponse>, Status> {
        self.async_runtime
            .block_on(self.async_grid_client.fetch_results())
    }

    ///
    pub fn submit_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        job_data: Vec<u8>,
    ) -> Result<Response<JobSubmitResponse>, Status> {
        self.async_runtime
            .block_on(
                self.async_grid_client
                    .submit_job(service_id, service_version, job_data),
            )
    }

    ///
    pub fn submit_result(
        &mut self,
        result: server_interface::Result,
    ) -> Result<Response<ResultSubmitResponse>, Status> {
        self.async_runtime
            .block_on(self.async_grid_client.submit_result(result))
    }
}
