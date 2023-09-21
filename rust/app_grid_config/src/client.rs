use grid_manager_interface::grid_manager_client::GridManagerClient;
use grid_manager_interface::{
    ClientId, RequestAcceptServiceLibrary, RequestGetStatus, RequestServerStart, RequestServerStop,
    RequestWorkerStart, RequestWorkerStop, ResponseAcceptServiceLibrary, ResponseGetStatus,
    ResponseServerStart, ResponseServerStop, ResponseWorkerStart, ResponseWorkerStop,
    ServerConfiguration, ServiceLibraryConfiguration, WorkerConfiguration,
};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

///
pub struct AsyncManagerClient {
    client_id: ClientId,
    grid_client: GridManagerClient<Channel>,
}

///
pub async fn connect_async_manager_client(
    server_address: &str,
) -> Result<AsyncManagerClient, Box<dyn std::error::Error>> {
    let grid_client = GridManagerClient::connect(format!("http://{}", server_address)).await?;

    // TODO
    /*
    // Register the client with the server.
    let register_client_response = grid_client
        .client_register(Request::new(RequestFromClientRegister {
            host_id: client_hostname().unwrap_or_default().to_lowercase(),
            user_id: user_id().unwrap_or_default().to_lowercase(),
        }))
        .await?;
     */

    Ok(AsyncManagerClient {
        // TODO
        client_id: 0, // register_client_response.get_ref().client_id,
        grid_client,
    })
}

impl AsyncManagerClient {
    ///
    pub async fn accept_service_library(
        &mut self,
        service_id: u32,
        service_version: u32,
        service_library_data: Vec<u8>,
    ) -> Result<Response<ResponseAcceptServiceLibrary>, Status> {
        self.grid_client
            .accept_service_library(Request::new(RequestAcceptServiceLibrary {
                client_id: self.client_id,
                service_library_data,
                service_library_configuration: Some(ServiceLibraryConfiguration {
                    service_id,
                    service_version,
                }),
            }))
            .await
    }

    ///
    pub async fn get_status(&mut self) -> Result<Response<ResponseGetStatus>, Status> {
        self.grid_client
            .get_status(Request::new(RequestGetStatus {
                client_id: self.client_id,
            }))
            .await
    }

    ///
    pub async fn start_server(
        &mut self,
        server_address: String,
    ) -> Result<Response<ResponseServerStart>, Status> {
        self.grid_client
            .start_server(Request::new(RequestServerStart {
                client_id: self.client_id,
                server_configuration: Some(ServerConfiguration { server_address }),
            }))
            .await
    }

    ///
    pub async fn start_worker(
        &mut self,
        server_address: String,
        service_id: u32,
        service_version: u32,
    ) -> Result<Response<ResponseWorkerStart>, Status> {
        self.grid_client
            .start_worker(Request::new(RequestWorkerStart {
                client_id: self.client_id,
                worker_configuration: Some(WorkerConfiguration {
                    server_address,
                    service_library_configuration: Some(ServiceLibraryConfiguration {
                        service_id,
                        service_version,
                    }),
                }),
            }))
            .await
    }

    ///
    pub async fn stop_server(
        &mut self,
        server_pid: u64,
    ) -> Result<Response<ResponseServerStop>, Status> {
        self.grid_client
            .stop_server(Request::new(RequestServerStop {
                client_id: self.client_id,
                server_pid,
            }))
            .await
    }

    ///
    pub async fn stop_worker(
        &mut self,
        worker_pid: u64,
    ) -> Result<Response<ResponseWorkerStop>, Status> {
        self.grid_client
            .stop_worker(Request::new(RequestWorkerStop {
                client_id: self.client_id,
                worker_pid,
            }))
            .await
    }
}
