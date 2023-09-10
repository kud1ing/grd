use grid_client::connect_async_grid_client;
use log::{error, info};
use std::env::args;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 2 {
        error!("Usage: grid-control <SERVER_ADDRESS>");
        exit(-1);
    }

    let server_address = &command_line_arguments[1];

    // Try to connect to the server.
    let mut grid_client = connect_async_grid_client(server_address, "monitor".to_string()).await?;

    // Try to get the server status.
    let status_response = grid_client.get_status().await?;
    let status_response = status_response.get_ref();

    // TODO
    println!("{:?}", status_response.status);

    Ok(())
}
