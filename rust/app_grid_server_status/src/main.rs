use grid_client::connect_async_grid_client;
use std::env::args;
use std::process::exit;

fn print_usage_and_stop() {
    println!("Usage: `grid-server-status <SERVER_ADDRESS>`");
    exit(-1);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 1 {
        print_usage_and_stop();
    }

    let server_address = &command_line_arguments[1];

    // Try to connect to the server.
    let mut grid_client = connect_async_grid_client(server_address, "monitor".to_string()).await?;

    // Try to get the server status.
    let status_response = grid_client.controller_get_status().await?;
    let status_response = status_response.get_ref();

    println!("{}", status_response.status);

    Ok(())
}
