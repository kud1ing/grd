use server_interface::{
    ClientId, GridClient, JobId, JobQuery, JobSubmitRequest, JobSubmitResponse,
    RegisterClientRequest, ResultFetchRequest, ResultFetchResponse, ResultSubmitRequest,
    ResultSubmitResponse, ServiceId, ServiceVersion, WorkerServerExchangeRequest,
    WorkerServerExchangeResponse,
};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};
#[cfg(not(target_os = "windows"))]
use users::{get_current_uid, get_user_by_uid};

///
pub struct AsyncGridClient {
    client_id: ClientId,
    grid_client: GridClient<Channel>,
}

///
fn client_hostname() -> Option<String> {
    hostname::get().ok()?.into_string().ok()
}

///
#[cfg(target_os = "windows")]
fn user_name() -> Option<String> {
    // TODO
    println!("TODO: `user_name()` on windows");
    Some("".to_string())
}

///
#[cfg(not(target_os = "windows"))]
fn user_name() -> Option<String> {
    let user = get_user_by_uid(get_current_uid())?;

    Some(user.name().to_str()?.to_string())
}

///
pub async fn connect_async_grid_client(
    server_address: &str,
    client_name: String,
) -> Result<AsyncGridClient, Box<dyn std::error::Error>> {
    let mut grid_client = GridClient::connect(format!("http://{}", server_address)).await?;

    // Register the client with the server.
    let register_client_response = grid_client
        .register_client(Request::new(RegisterClientRequest {
            host_name: client_hostname().unwrap_or_default().to_lowercase(),
            user_name: user_name().unwrap_or_default().to_lowercase(),
            client_name,
        }))
        .await?;

    Ok(AsyncGridClient {
        client_id: register_client_response.get_ref().client_id,
        grid_client,
    })
}

impl AsyncGridClient {
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
    pub async fn worker_server_exchange(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        result_from_worker: Option<server_interface::Result>,
    ) -> Result<Response<WorkerServerExchangeResponse>, Status> {
        self.grid_client
            .worker_server_exchange(Request::new(WorkerServerExchangeRequest {
                query_job_from_server: Some(JobQuery {
                    service_id,
                    service_version,
                }),
                result_from_worker,
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
