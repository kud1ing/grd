pub mod grid_manager_interface {
    // Include the generated rust module.
    tonic::include_proto!("grid_manager_interface");
}

pub use grid_manager_interface::grid_manager_server::{GridManager, GridManagerServer};
pub use grid_manager_interface::*;
