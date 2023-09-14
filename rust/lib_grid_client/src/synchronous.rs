use crate::{connect_async_grid_client, AsyncGridClient};
use grid_server_interface::{
    ResponseToClientJobSubmit, ResponseToClientResultFetch, ResponseToWorkerExchange,
    ResponseToWorkerResultSubmit, ServiceId, ServiceVersion,
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
    client_id: String,
) -> Result<SyncGridClient, Box<dyn std::error::Error>> {
    let async_runtime = Builder::new_multi_thread().enable_all().build()?;

    // Connect the grid client.
    let async_grid_client =
        async_runtime.block_on(connect_async_grid_client(server_address, client_id))?;

    Ok(SyncGridClient {
        async_grid_client,
        async_runtime,
    })
}

impl SyncGridClient {
    ///
    pub fn client_fetch_results(
        &mut self,
    ) -> Result<Response<ResponseToClientResultFetch>, Status> {
        self.async_runtime
            .block_on(self.async_grid_client.client_fetch_results())
    }

    ///
    pub fn client_submit_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        job_data: Vec<u8>,
    ) -> Result<Response<ResponseToClientJobSubmit>, Status> {
        self.async_runtime
            .block_on(self.async_grid_client.client_submit_job(
                service_id,
                service_version,
                job_data,
            ))
    }

    ///
    pub fn worker_submit_result(
        &mut self,
        result: grid_server_interface::Result,
    ) -> Result<Response<ResponseToWorkerResultSubmit>, Status> {
        self.async_runtime
            .block_on(self.async_grid_client.worker_submit_result(result))
    }

    ///
    pub fn worker_server_exchange(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        result_from_worker: Option<grid_server_interface::Result>,
    ) -> Result<Response<ResponseToWorkerExchange>, Status> {
        self.async_runtime
            .block_on(self.async_grid_client.worker_server_exchange(
                service_id,
                service_version,
                result_from_worker,
            ))
    }
}
