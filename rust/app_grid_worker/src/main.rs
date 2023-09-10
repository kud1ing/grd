use grid_client::connect_async_grid_client;
use grid_server_interface::ServiceId;
use libc;
use log::{error, info};
use std::env::args;
use std::process::exit;
use std::thread;
use std::time::Duration;

/// The C-ABI signature of the service function.
type ServiceFunction = unsafe extern "C" fn(
    data_in: *const libc::c_void,
    size_in: libc::c_longlong,
    data_out: *mut libc::c_void,
    size_out: libc::c_longlong,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 6 {
        error!(
            "Usage: grid-worker <SERVER_ADDRESS> <WORKER_NAME> <SERVICE_ID> <SERVICE_VERSION>\
             <PATH_SERVICE_FUNCTION>"
        );
        exit(-1);
    }

    let server_address = &command_line_arguments[1];
    let worker_name = &command_line_arguments[2];
    let service_id = command_line_arguments[3].parse()?;
    let service_version = command_line_arguments[4].parse()?;
    let path_service_library = &command_line_arguments[5];

    // Try to load the service library.
    let service_library = unsafe { libloading::Library::new(path_service_library)? };

    // Try to get the service function within the service library.
    let _service_function: libloading::Symbol<ServiceFunction> =
        unsafe { service_library.get(b"service_function")? };

    // Try to connect to the server.
    let mut grid_client =
        connect_async_grid_client(server_address, worker_name.to_string()).await?;

    info!("Processing jobs ...");

    let mut result: Option<grid_server_interface::Result> = None;

    loop {
        // Try to fetch a job from the server and maybe also send a result to the server.
        let worker_server_exchange_response = grid_client
            .worker_server_exchange(service_id, service_version, result.clone())
            .await?;

        let worker_server_exchange_response = worker_server_exchange_response.get_ref();

        let job =
            // There is a new job from the server.
            if let Some(job) = &worker_server_exchange_response.job {
                job
            }
            // There is no new job.
            else {
                result = None;
                thread::sleep(Duration::from_millis(1000));
                continue;
            };

        // TODO: Process the job with `service_function()`.
        result = None;
    }

    info!("Done.");

    Ok(())
}
