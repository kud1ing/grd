mod client;

use crate::client::connect_async_manager_client;
use std::env::args;
use std::fs::read;
use std::process::exit;

fn print_usage_and_stop() {
    println!("Usage: `grid-config <MANAGER_ADDRESS> <COMMAND>`");
    println!("Example:");
    println!("  grid-config 127.0.0.1:50000 upload 0 42 service_library.dll");
    println!("  grid-config 127.0.0.1:50000 status");
    println!("  grid-config 127.0.0.1:50000 start-server 127.0.0.1:60000");
    println!("  grid-config 127.0.0.1:50000 start-worker 127.0.0.1:60000 0 42");
    println!("  grid-config 127.0.0.1:50000 stop-server 666");
    println!("  grid-config 127.0.0.1:50000 stop-worker 777");
    exit(-1);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() <= 2 {
        print_usage_and_stop();
    }

    let manager_address = &command_line_arguments[1];
    let command = &command_line_arguments[2];

    // Try to connect to the server.
    let mut manager_client = connect_async_manager_client(manager_address).await?;

    match command.as_ref() {
        "status" => {
            let status_response = manager_client.get_status().await?;
            let status_response = status_response.get_ref();

            println!("Servers:");

            for server_status in &status_response.server_status {
                let server_address = server_status
                    .server_configuration
                    .clone()
                    .map(|sc| sc.server_address)
                    .unwrap_or_else(|| "n/a".to_string());

                println!("{} ({})", server_address, server_status.server_pid);
            }

            println!("Workers:");

            for worker_status in &status_response.worker_status {
                let server_address = worker_status
                    .worker_configuration
                    .clone()
                    .map(|sc| sc.server_address)
                    .unwrap_or_else(|| "n/a".to_string());

                println!("{} ({})", server_address, worker_status.worker_pid);
            }
        }
        "start-server" => {
            // Too few command line arguments are given.
            if command_line_arguments.len() <= 3 {
                print_usage_and_stop();
            }

            let server_address = command_line_arguments[3].clone();

            let response = manager_client.start_server(server_address).await?;
            let response = response.get_ref();

            if let Some(error_message) = &response.error_message {
                eprintln!("Error from grid manager: {}", error_message);
            }
        }
        "start-worker" => {
            // Too few command line arguments are given.
            if command_line_arguments.len() <= 5 {
                print_usage_and_stop();
            }

            let server_address = command_line_arguments[3].clone();
            let service_id = command_line_arguments[4].parse()?;
            let service_version = command_line_arguments[5].parse()?;

            let response = manager_client
                .start_worker(server_address, service_id, service_version)
                .await?;
            let response = response.get_ref();

            if let Some(error_message) = &response.error_message {
                eprintln!("Error from grid manager: {}", error_message);
            }
        }
        "stop-server" => {
            // Too few command line arguments are given.
            if command_line_arguments.len() <= 3 {
                print_usage_and_stop();
            }

            let server_pid = command_line_arguments[3].parse()?;

            let response = manager_client.stop_server(server_pid).await?;
            let response = response.get_ref();

            if let Some(error_message) = &response.error_message {
                eprintln!("Error from grid manager: {}", error_message);
            }
        }
        "stop-worker" => {
            // Too few command line arguments are given.
            if command_line_arguments.len() <= 3 {
                print_usage_and_stop();
            }

            let worker_pid = command_line_arguments[3].parse()?;

            let response = manager_client.stop_worker(worker_pid).await?;
            let response = response.get_ref();

            if let Some(error_message) = &response.error_message {
                eprintln!("Error from grid manager: {}", error_message);
            }
        }
        "upload" => {
            // Too few command line arguments are given.
            if command_line_arguments.len() <= 5 {
                print_usage_and_stop();
            }

            let service_id = command_line_arguments[3].parse()?;
            let service_version = command_line_arguments[4].parse()?;
            let service_library_path = &command_line_arguments[5];

            // Try to read the service library.
            let service_library_data = read(service_library_path)?;

            let response = manager_client
                .accept_service_library(service_id, service_version, service_library_data)
                .await?;
            let response = response.get_ref();

            if let Some(error_message) = &response.error_message {
                eprintln!("Error from grid manager: {}", error_message);
            }
        }
        &_ => {
            eprintln!("Unhandled grid manager command");
        }
    }

    Ok(())
}
