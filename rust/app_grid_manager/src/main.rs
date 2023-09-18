#[macro_use]
extern crate log;

use grid_manager_interface::{
    GridManager, GridManagerServer, RequestGetStatus, RequestServerStop, RequestWorkerStart,
    RequestWorkerStop, ResponseGetStatus, ResponseServerStop, ResponseWorkerStart,
    ResponseWorkerStop, WorkerConfiguration,
};
use std::env::{args, current_exe};
use std::path::Path;
use std::process::{exit, Command};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

///
const GRID_SERVER_EXECUTABLE_NAME: &str = "grid-server.exe";

///
const GRID_WORKER_EXECUTABLE_NAME: &str = "grid-worker.exe";

/// Starts a grid worker process with the given configuration.
fn start_worker(worker_configuration: &WorkerConfiguration) {
    // Determine the path to the current grid manager executable.
    let grid_manager_executable_path = match current_exe() {
        Ok(path_to_executable_of_current_grid_worker) => path_to_executable_of_current_grid_worker,
        Err(error) => {
            error!("Could not get path to executable of the current grid manager: {error}");
            return;
        }
    };

    // Get grid base path.
    let grid_manager_executable_base_path = match grid_manager_executable_path.parent() {
        None => {
            error!("Could not get base path of the current grid manager executable");
            return;
        }
        Some(grid_manager_executable_base_path) => grid_manager_executable_base_path,
    };

    // Construct the grid worker executable path.
    let grid_worker_executable_path =
        grid_manager_executable_base_path.join(GRID_WORKER_EXECUTABLE_NAME);

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
        info!("Started grid worker");
    }
}

// =================================================================================================

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
        request: Request<RequestGetStatus>,
    ) -> Result<Response<ResponseGetStatus>, Status> {
        let request = request.get_ref();

        // TODO: Determine running grid servers and grid workers.
        todo!()
    }

    async fn start_worker(
        &self,
        request: Request<RequestWorkerStart>,
    ) -> Result<Response<ResponseWorkerStart>, Status> {
        let request = request.get_ref();

        if let Some(worker_configuration) = &request.worker_configuration {
            start_worker(worker_configuration);
        }

        Ok(Response::new(ResponseWorkerStart {}))
    }

    async fn stop_server(
        &self,
        request: Request<RequestServerStop>,
    ) -> Result<Response<ResponseServerStop>, Status> {
        let request = request.get_ref();

        todo!()
    }

    async fn stop_worker(
        &self,
        request: Request<RequestWorkerStop>,
    ) -> Result<Response<ResponseWorkerStop>, Status> {
        let request = request.get_ref();

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
