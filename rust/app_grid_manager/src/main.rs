#[macro_use]
extern crate log;

use grid_manager_interface::{
    GridManager, GridManagerServer, RequestAcceptServiceLibrary, RequestGetStatus,
    RequestServerStart, RequestServerStop, RequestWorkerStart, RequestWorkerStop,
    ResponseAcceptServiceLibrary, ResponseGetStatus, ResponseServerStart, ResponseServerStop,
    ResponseWorkerStart, ResponseWorkerStop, ServerConfiguration, ServerStatus,
    ServiceLibraryConfiguration, WorkerConfiguration, WorkerStatus,
};
use lazy_static::lazy_static;
use std::env::{args, current_exe};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::sync::Mutex;
use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

lazy_static! {
    // Determine the parent path of the current executable's path.
    static ref GRID_BASE_PATH: PathBuf = current_exe().unwrap().parent().unwrap().to_path_buf();
    static ref LIBRARIES_PATH: PathBuf = {
        let mut path = GRID_BASE_PATH.clone();
        path.push("libraries");
        path
    };
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

const SERVICE_LIBRARY_NAME: &str = if cfg!(unix) {
    "service_library.so"
} else {
    "service_library.dll"
};

///
fn pid_from_u64(process_id: u64) -> Pid {
    Pid::from(process_id as usize)
}

///
fn process_id_from_process(process: &Process) -> u64 {
    process.pid().as_u32() as u64
}

///
fn service_library_path(service_id: u32, service_version: u32) -> PathBuf {
    let mut path = LIBRARIES_PATH.clone();
    path.push(service_id.to_string());
    path.push(service_version.to_string());
    path.push(SERVICE_LIBRARY_NAME);

    path
}

/// Starts a grid server process with the given configuration.
fn start_server(server_configuration: &ServerConfiguration) {
    // Construct the grid server executable path.
    let grid_server_executable_path = GRID_BASE_PATH.clone().join(GRID_SERVER_EXECUTABLE_NAME);

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
    // Construct the grid worker executable path.
    let grid_worker_executable_path = GRID_BASE_PATH.clone().join(GRID_WORKER_EXECUTABLE_NAME);

    // A service library configuration is given.
    if let Some(service_library_configuration) = &worker_configuration.service_library_configuration
    {
        // Try to start the grid worker according to the given configuration.
        if let Err(error) = Command::new(grid_worker_executable_path)
            .args([
                worker_configuration.server_address.clone(),
                service_library_configuration.service_id.to_string(),
                service_library_configuration.service_version.to_string(),
                service_library_path(
                    service_library_configuration.service_id,
                    service_library_configuration.service_version,
                )
                .into_os_string()
                .into_string()
                .unwrap(),
            ])
            .spawn()
        {
            error!("Could not start grid worker: {error}");
        } else {
            info!("Started grid worker");
        }
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
    async fn accept_service_library(
        &self,
        request: Request<RequestAcceptServiceLibrary>,
    ) -> Result<Response<ResponseAcceptServiceLibrary>, Status> {
        let request = request.get_ref();
        // TODO handle `request.client_id`?

        if let Some(service_library_configuration) = &request.service_library_configuration {
            // Determine the service library path.
            let service_library_path = service_library_path(
                service_library_configuration.service_id,
                service_library_configuration.service_version,
            );

            // The service library path has a parent.
            if let Some(parent) = service_library_path.parent() {
                // The service library path does not exist.
                if !parent.exists() {
                    // Try to create the parent directory.
                    match create_dir_all(parent) {
                        Ok(_) => {
                            info!(
                                "Created the service library directory \"{}\"",
                                parent.display()
                            );
                        }
                        Err(error) => {
                            let error_message = format!(
                                "Could not create the service library directory \"{}\": {error}",
                                parent.display()
                            );

                            error!("{}", &error_message);

                            return Ok(Response::new(ResponseAcceptServiceLibrary {
                                error_message: Some(error_message),
                            }));
                        }
                    }
                }

                // Write the service library.
                {
                    let mut file = File::create(&service_library_path)?;

                    // Try to write the service library data.
                    match file.write_all(&request.service_library_data) {
                        Ok(_) => {
                            info!(
                                "Wrote the service library \"{}\"",
                                service_library_path.display()
                            );

                            return Ok(Response::new(ResponseAcceptServiceLibrary {
                                error_message: None,
                            }));
                        }
                        Err(error) => {
                            let error_message = format!(
                                "Could not write the service library \"{}\": {error}",
                                parent.display()
                            );

                            error!("{}", &error_message);

                            return Ok(Response::new(ResponseAcceptServiceLibrary {
                                error_message: Some(error_message),
                            }));
                        }
                    }
                }
            }
            // The service library path has no parent.
            else {
                let error_message =
                    "Could not determine the service library directory path".to_string();

                error!("{}", &error_message);

                return Ok(Response::new(ResponseAcceptServiceLibrary {
                    error_message: Some(error_message),
                }));
            }

            let error_message = "`accept_service_library()`: Unhandled".to_string();

            error!("{}", &error_message);

            return Ok(Response::new(ResponseAcceptServiceLibrary {
                error_message: Some(error_message),
            }));
        }

        let error_message = "No service library configuration given".to_string();

        error!("{}", &error_message);

        Ok(Response::new(ResponseAcceptServiceLibrary {
            error_message: Some(error_message),
        }))
    }

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
        let worker_status = {
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

                worker_status.push(WorkerStatus {
                    worker_pid: process_id_from_process(process),
                    worker_configuration: Some(WorkerConfiguration {
                        server_address,
                        service_library_configuration: Some(ServiceLibraryConfiguration {
                            service_id,
                            service_version,
                        }),
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

            return Ok(Response::new(ResponseServerStart {
                error_message: None,
            }));
        }

        Ok(Response::new(ResponseServerStart {
            error_message: Some("No server configuration given".to_string()),
        }))
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

            return Ok(Response::new(ResponseWorkerStart {
                error_message: None,
            }));
        }

        Ok(Response::new(ResponseWorkerStart {
            error_message: Some("No worker configuration given".to_string()),
        }))
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
            return Ok(Response::new(ResponseServerStop {
                error_message: None,
            }));
        }

        Ok(Response::new(ResponseServerStop {
            error_message: Some("No server process with the given process ID".to_string()),
        }))
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
            return Ok(Response::new(ResponseWorkerStop {
                error_message: None,
            }));
        }

        Ok(Response::new(ResponseWorkerStop {
            error_message: Some("No worker process with the given process ID".to_string()),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 2 {
        error!("Please pass the grid manager socket address.");
        exit(-1);
    }

    // Construct the socket address from the command line argument,
    let socket_address = command_line_arguments[1].parse()?;

    info!("Running the grid manager on \"{}\" ...", socket_address);

    Server::builder()
        .add_service(
            // Lift the 4 MB limit so that bigger service libraries can be uploaded.
            GridManagerServer::new(GridManagerImpl::new()).max_decoding_message_size(usize::MAX),
        )
        .serve(socket_address)
        .await?;

    Ok(())
}
