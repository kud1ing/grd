use server_interface::{
    ClientId, GridClient, JobFetchRequest, JobFetchResponse, JobSubmitRequest, JobSubmitResponse,
    ResultFetchRequest, ResultFetchResponse, ResultSubmitRequest, ResultSubmitResponse, ServiceId,
    ServiceVersion,
};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

///
pub struct AsyncGridClient {
    client_id: ClientId,
    grid_client: GridClient<Channel>,
}

///
pub async fn connect_async_grid_client(
    host_name: &str,
    port: u32,
) -> Result<AsyncGridClient, Box<dyn std::error::Error>> {
    let grid_client = GridClient::connect(format!("http://{}:{}", host_name, port)).await?;

    // TODO
    println!("TODO `connect_async_grid_client()`: request a client ID");
    let client_id = 0;

    Ok(AsyncGridClient {
        client_id,
        grid_client,
    })
}

impl AsyncGridClient {
    ///
    pub async fn fetch_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
    ) -> Result<Response<JobFetchResponse>, Status> {
        self.grid_client
            .fetch_job(Request::new(JobFetchRequest {
                service_id,
                service_version,
            }))
            .await
    }

    ///
    pub async fn fetch_results(&mut self) -> Result<Response<ResultFetchResponse>, Status> {
        self.grid_client
            .fetch_results(Request::new(ResultFetchRequest {
                client_id: self.client_id,
            }))
            .await
    }

    ///
    pub async fn submit_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        job_data: Vec<u8>,
    ) -> Result<Response<JobSubmitResponse>, Status> {
        self.grid_client
            .submit_job(Request::new(JobSubmitRequest {
                client_id: self.client_id,
                job_data,
                service_id,
                service_version,
            }))
            .await
    }

    ///
    pub async fn submit_result(
        &mut self,
        result: server_interface::Result,
    ) -> Result<Response<ResultSubmitResponse>, Status> {
        self.grid_client
            .submit_result(Request::new(ResultSubmitRequest {
                result: Some(result),
            }))
            .await
    }
}
