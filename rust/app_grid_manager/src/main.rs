#[macro_use]
extern crate log;

use grid_manager_interface::{
    GridManager, GridManagerServer, RequestGetStatus, RequestServerStop, RequestWorkerStart,
    RequestWorkerStop, ResponseGetStatus, ResponseServerStop, ResponseWorkerStart,
    ResponseWorkerStop, WorkerConfiguration,
};
use std::env::{args, current_exe};
use std::process::{exit, Command};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// Starts a grid worker process with the given configuration.
fn start_worker(worker_configuration: &WorkerConfiguration) {
    // Determine the path to the executable of the current grid worker
    // TODO: get the base path and call tbe worker
    let grid_worker_executable_path = match current_exe() {
        Ok(path_to_executable_of_current_grid_worker) => path_to_executable_of_current_grid_worker,
        Err(error) => {
            error!("Could not get path to executable of the current grid worker: {error}");
            return;
        }
    };

    // Try to start the grid worker according to the given configuration.
    if let Err(error) = Command::new(grid_worker_executable_path)
        .args([
            worker_configuration.server_address.clone(),
            worker_configuration.service_id.to_string(),
            worker_configuration.service_version.to_string(),
            worker_configuration.service_library_path.clone(),
        ])
        .spawn()
    {
        error!("Could not start grid worker: {error}");
    } else {
        info!("Started worker");
    }
}

/// The grid manager.
pub struct GridManagerImpl {}

impl GridManagerImpl {
    ///
    fn new() -> Self {
        GridManagerImpl {}
    }
}

/// The implementation of the manager interface for the manager.
#[tonic::async_trait]
impl GridManager for GridManagerImpl {
    async fn get_status(
        &self,
        _request: Request<RequestGetStatus>,
    ) -> Result<Response<ResponseGetStatus>, Status> {
        // TODO: Determine running grid servers and grid workers.
        todo!()
    }

    async fn start_worker(
        &self,
        _request: Request<RequestWorkerStart>,
    ) -> Result<Response<ResponseWorkerStart>, Status> {
        todo!()
    }

    async fn stop_server(
        &self,
        _request: Request<RequestServerStop>,
    ) -> Result<Response<ResponseServerStop>, Status> {
        todo!()
    }

    async fn stop_worker(
        &self,
        _request: Request<RequestWorkerStop>,
    ) -> Result<Response<ResponseWorkerStop>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 2 {
        error!("Please pass the manager socket address.");
        exit(-1);
    }

    // Construct the socket address from the command line argument,
    let socket_address = command_line_arguments[1].parse()?;

    info!("Running the manager on \"{}\" ...", socket_address);

    Server::builder()
        .add_service(GridManagerServer::new(GridManagerImpl::new()))
        .serve(socket_address)
        .await?;

    Ok(())
}
