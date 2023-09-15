fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("grid_server_interface.proto")?;
    Ok(())
}
