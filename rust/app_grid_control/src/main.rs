use grid_client::connect_async_grid_client;
use log::error;
use std::env::args;
use std::process::exit;

fn print_usage_and_stop() {
    println!("Usage: `grid-control <SERVER_ADDRESS> <COMMANDS>`");
    println!("Examples:");
    println!("  `grid-control 127.0.0.1 status`        : asks the grid server for its status and prints it");
    println!("  `grid-control 127.0.0.1 stop`          : asks the grid server to stop");
    println!("  `grid-control 127.0.0.1 worker 0 stop` : asks the grid worker with the client ID 0 to stop");
    exit(-1);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 3 {
        print_usage_and_stop();
    }

    let server_address = &command_line_arguments[1];
    let command = &command_line_arguments[2];

    // Try to connect to the server.
    let mut grid_client = connect_async_grid_client(server_address, "monitor".to_string()).await?;

    // Get the server status.
    if command == "status" {
        // Try to get the server status.
        let status_response = grid_client.controller_get_status().await?;
        let status_response = status_response.get_ref();

        // TODO
        println!("{:?}", status_response.status);

        return Ok(());
    }

    // Stopping the server.
    if command == "stop" {
        // Try to get the server status.
        let _ = grid_client.controller_stop_server().await?;

        return Ok(());
    }

    // Commands for a worker.
    if command == "worker" {
        // Too few command line arguments are given.
        if command_line_arguments.len() < 4 {
            print_usage_and_stop();
        }

        let worker_client_id = &command_line_arguments[3].parse()?;
        let command = &command_line_arguments[4];

        if command == "stop" {
            let _ = grid_client
                .controller_stop_worker(*worker_client_id)
                .await?;

            return Ok(());
        }
    }

    error!("Unhandled arguments");

    Ok(())
}
