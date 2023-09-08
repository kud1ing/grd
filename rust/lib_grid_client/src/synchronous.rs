use crate::{connect_async_grid_client, AsyncGridClient};
use server_interface::{
    JobId, JobSubmitResponse, ResultFetchResponse, ResultSubmitResponse, ServiceId, ServiceVersion,
    WorkerServerExchangeResponse,
};
use tokio::runtime::{Builder, Runtime};
use tonic::{Response, Status};

///
pub struct SyncGridClient {
    async_grid_client: AsyncGridClient,
    async_runtime: Runtime,
}

///
pub fn connect_sync_grid_client(
    server_address: &str,
    client_name: String,
) -> Result<SyncGridClient, Box<dyn std::error::Error>> {
    let async_runtime = Builder::new_multi_thread().enable_all().build()?;

    // Connect the grid client.
    let async_grid_client =
        async_runtime.block_on(connect_async_grid_client(server_address, client_name))?;

    Ok(SyncGridClient {
        async_grid_client,
        async_runtime,
    })
}

impl SyncGridClient {
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
    pub fn worker_server_exchange(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        result_from_worker: Option<server_interface::Result>,
    ) -> Result<Response<WorkerServerExchangeResponse>, Status> {
        self.async_runtime
            .block_on(self.async_grid_client.worker_server_exchange(
                service_id,
                service_version,
                result_from_worker,
            ))
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
