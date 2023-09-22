# grd

A simple computational grid server, using [tonic](https://github.com/hyperium/tonic) ([gRPC](https://grpc.io/) and
[Protocol Buffers 3](https://developers.google.com/protocol-buffers/docs/proto3)).

While the server is written in [Rust](https://www.rust-lang.org), clients and workers can be implemented
in any language:
* bring your own serialization format: the job data and result data are exchanged as raw bytes
* a Python client and worker is provided (using [PyO3](https://pyo3.rs))

## Compilation dependencies

See tonic's [dependencies](https://github.com/hyperium/tonic#dependencies)

## Backlog

* [ ] add tracing to the server: add a member with timestamps of the job per `job_id`
* [ ] `app_grid_manager`: extend the status with the available service libraries (`LIBRARIES_PATH`) with creation/modification date
* [ ] Grid Config: make it possible to stop all grid servers
* [ ] Grid Config: make it possible to stop all grid workers
* make it possible to remove libraries:
  * [ ] `grid_manager_interface.proto`
  * [ ] `app_grid_manager`
  * [ ] `app_grid_configr`
    * [ ] `client`
* [ ] Worker: Windows: signal kill from Grid Manager is not handled. Ctrl-C is, though.
* [ ] Worker: call a service library function to free the result memory
* [ ] Manager: Mac: vanished processes still show up in status
  * https://github.com/GuillaumeGomez/sysinfo/issues/686
* [ ] `client_cpp`: create a sync C++ client
  * [ ] how to check-in/distribute the generated `.cc` and `.h` files?
  * [ ] expose `SyncGridClient`'s methods
    * https://cxx.rs/extern-rust.html#methods
* [ ] expose an async Python Client
  * https://pyo3.rs/v0.13.2/ecosystem/async-await.html#awaiting-a-rust-future-in-python
* [ ] `client_cpp`: create an async C++ client
* Release building:
  * [ ] is currently not triggered by pushing a tag
  * [ ] `release.yml`: `${{ env.GRD_VERSION }}` and `${{ env.RUNNER_OS }}` are empty and thus currently hardcoded
