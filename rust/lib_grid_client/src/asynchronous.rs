use grid_server_interface::grid_server_interface::{
    RequestFromControllerStatusGet, RequestFromControllerWorkerStop, ResponseToControllerStatusGet,
    ResponseToControllerWorkerStop,
};
use grid_server_interface::{
    ClientId, GridClient, JobQuery, RequestFromClientJobSubmit, RequestFromClientRegister,
    RequestFromClientResultFetch, RequestFromControllerServerStop, RequestFromWorkerExchange,
    RequestFromWorkerResultSubmit, ResponseToClientJobSubmit, ResponseToClientResultFetch,
    ResponseToControllerServerStop, ResponseToWorkerExchange, ResponseToWorkerResultSubmit,
    ServiceId, ServiceVersion,
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
fn user_id() -> Option<String> {
    // TODO
    println!("TODO: `user_id()` on windows");
    Some("".to_string())
}

///
#[cfg(not(target_os = "windows"))]
fn user_id() -> Option<String> {
    let user = get_user_by_uid(get_current_uid())?;

    Some(user.name().to_str()?.to_string())
}

///
pub async fn connect_async_grid_client(
    server_address: &str,
    client_description: String,
) -> Result<AsyncGridClient, Box<dyn std::error::Error>> {
    let mut grid_client = GridClient::connect(format!("http://{}", server_address)).await?;

    // Register the client with the server.
    let register_client_response = grid_client
        .client_register(Request::new(RequestFromClientRegister {
            client_description,
            host_id: client_hostname().unwrap_or_default().to_lowercase(),
            user_id: user_id().unwrap_or_default().to_lowercase(),
        }))
        .await?;

    Ok(AsyncGridClient {
        client_id: register_client_response.get_ref().client_id,
        grid_client,
    })
}

impl AsyncGridClient {
    ///
    pub async fn client_fetch_results(
        &mut self,
    ) -> Result<Response<ResponseToClientResultFetch>, Status> {
        self.grid_client
            .client_fetch_results(Request::new(RequestFromClientResultFetch {
                client_id: self.client_id,
            }))
            .await
    }

    ///
    pub async fn controller_get_status(
        &mut self,
    ) -> Result<Response<ResponseToControllerStatusGet>, Status> {
        self.grid_client
            .controller_get_status(Request::new(RequestFromControllerStatusGet {
                client_id: self.client_id,
            }))
            .await
    }

    ///
    pub async fn controller_stop_server(
        &mut self,
    ) -> Result<Response<ResponseToControllerServerStop>, Status> {
        self.grid_client
            .controller_stop_server(Request::new(RequestFromControllerServerStop {
                client_id: self.client_id,
            }))
            .await
    }

    ///
    pub async fn controller_stop_worker(
        &mut self,
        worker_client_id: ClientId,
    ) -> Result<Response<ResponseToControllerWorkerStop>, Status> {
        self.grid_client
            .controller_stop_worker(Request::new(RequestFromControllerWorkerStop {
                client_id: self.client_id,
                worker_client_id,
            }))
            .await
    }

    ///
    pub async fn client_submit_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        job_data: Vec<u8>,
    ) -> Result<Response<ResponseToClientJobSubmit>, Status> {
        self.grid_client
            .client_submit_job(Request::new(RequestFromClientJobSubmit {
                client_id: self.client_id,
                job_data,
                service_id,
                service_version,
            }))
            .await
    }

    ///
    pub async fn worker_submit_result(
        &mut self,
        result: grid_server_interface::Result,
    ) -> Result<Response<ResponseToWorkerResultSubmit>, Status> {
        self.grid_client
            .worker_submit_result(Request::new(RequestFromWorkerResultSubmit {
                client_id: self.client_id,
                result: Some(result),
            }))
            .await
    }

    ///
    pub async fn worker_server_exchange(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        result_from_worker: Option<grid_server_interface::Result>,
    ) -> Result<Response<ResponseToWorkerExchange>, Status> {
        self.grid_client
            .worker_server_exchange(Request::new(RequestFromWorkerExchange {
                client_id: self.client_id,
                query_job_from_server: Some(JobQuery {
                    service_id,
                    service_version,
                }),
                result_from_worker,
            }))
            .await
    }
}
