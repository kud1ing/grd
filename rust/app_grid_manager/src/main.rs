#[macro_use]
extern crate log;

use grid_manager_interface::{
    GridManager, GridManagerServer, RequestGetStatus, RequestServerStart, RequestServerStop,
    RequestWorkerStart, RequestWorkerStop, ResponseGetStatus, ResponseServerStart,
    ResponseServerStop, ResponseWorkerStart, ResponseWorkerStop, ServerConfiguration, ServerStatus,
    WorkerConfiguration, WorkerStatus,
};
use lazy_static::lazy_static;
use std::env::{args, current_exe};
use std::process::{exit, Command};
use std::sync::Mutex;
use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

lazy_static! {
    static ref SYSTEM: Mutex<System> = Mutex::new(System::new());
}

const GRID_SERVER_EXECUTABLE_NAME: &str = if cfg!(unix) {
    "grid-server"
} else {
    "grid-server.exe"
};

const GRID_WORKER_EXECUTABLE_NAME: &str = if cfg!(unix) {
    "grid-worker"
} else {
    "grid-worker.exe"
};

///
fn pid_from_u64(process_id: u64) -> Pid {
    Pid::from(process_id as usize)
}

///
fn process_id_from_process(process: &Process) -> u64 {
    process.pid().as_u32() as u64
}

/// Starts a grid server process with the given configuration.
fn start_server(server_configuration: &ServerConfiguration) {
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

    // Construct the grid server executable path.
    let grid_server_executable_path =
        grid_manager_executable_base_path.join(GRID_SERVER_EXECUTABLE_NAME);

    // Try to start the grid server according to the given configuration.
    if let Err(error) = Command::new(grid_server_executable_path)
        .args([server_configuration.server_address.clone()])
        .spawn()
    {
        error!("Could not start grid server: {error}");
    } else {
        info!("Started grid server");
    }
}

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
        _request: Request<RequestGetStatus>,
    ) -> Result<Response<ResponseGetStatus>, Status> {
        // TODO handle `request.client_id`?

        let mut system = SYSTEM.lock().unwrap();
        system.refresh_processes();

        // Determine the running grid server processes.
        let server_status = {
            let mut server_status = vec![];

            // Iterate over all grid server processes.
            for process in system.processes_by_exact_name(GRID_SERVER_EXECUTABLE_NAME) {
                // Try to get the server address.
                let server_address = process
                    .cmd()
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "n/a".to_string());

                server_status.push(ServerStatus {
                    server_pid: process_id_from_process(process),
                    server_configuration: Some(ServerConfiguration { server_address }),
                });
            }

            server_status
        };

        // Determine the running grid server processes.
        let mut worker_status = {
            let mut worker_status = vec![];

            // Iterate over all grid worker processes.
            for process in system.processes_by_exact_name(GRID_WORKER_EXECUTABLE_NAME) {
                // Try to get the server address.
                let server_address = process
                    .cmd()
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "n/a".to_string());

                let service_id = process
                    .cmd()
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| "0".to_string())
                    .parse()
                    .unwrap_or(0);

                let service_version = process
                    .cmd()
                    .get(3)
                    .cloned()
                    .unwrap_or_else(|| "0".to_string())
                    .parse()
                    .unwrap_or(0);

                let service_library_path = process
                    .cmd()
                    .get(4)
                    .cloned()
                    .unwrap_or_else(|| "n/a".to_string());

                worker_status.push(WorkerStatus {
                    worker_pid: process_id_from_process(process),
                    worker_configuration: Some(WorkerConfiguration {
                        server_address,
                        service_id,
                        service_library_path,
                        service_version,
                    }),
                });
            }

            worker_status
        };

        Ok(Response::new(ResponseGetStatus {
            server_status,
            worker_status,
        }))
    }

    async fn start_server(
        &self,
        request: Request<RequestServerStart>,
    ) -> Result<Response<ResponseServerStart>, Status> {
        let request = request.get_ref();
        // TODO handle `request.client_id`?

        // A server configuration is given.
        if let Some(server_configuration) = &request.server_configuration {
            start_server(server_configuration);
        }

        Ok(Response::new(ResponseServerStart {}))
    }

    async fn start_worker(
        &self,
        request: Request<RequestWorkerStart>,
    ) -> Result<Response<ResponseWorkerStart>, Status> {
        let request = request.get_ref();
        // TODO handle `request.client_id`?

        // A worker configuration is given.
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
        // TODO handle `request.client_id`?
        let server_pid = pid_from_u64(request.server_pid);

        let mut system = SYSTEM.lock().unwrap();
        system.refresh_processes();

        // There is a process with the given PID.
        if let Some(server_process) = system.process(server_pid) {
            // Try to kill the grid server process.
            server_process.kill();
        }

        Ok(Response::new(ResponseServerStop {}))
    }

    async fn stop_worker(
        &self,
        request: Request<RequestWorkerStop>,
    ) -> Result<Response<ResponseWorkerStop>, Status> {
        let request = request.get_ref();
        // TODO handle `request.client_id`?
        let worker_pid = pid_from_u64(request.worker_pid);

        let mut system = SYSTEM.lock().unwrap();
        system.refresh_processes();

        // There is a process with the given PID.
        if let Some(worker_process) = system.process(worker_pid) {
            // Try to kill the grid worker process.
            worker_process.kill();
        }

        Ok(Response::new(ResponseWorkerStop {}))
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
