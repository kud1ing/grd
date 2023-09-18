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

* `app_grid_manager`: 
  * implement `get_status()`:
    * https://docs.rs/sysinfo/latest/sysinfo/trait.SystemExt.html#method.processes_by_exact_name
    * https://docs.rs/sysinfo/latest/sysinfo/trait.ProcessExt.html#tymethod.pid
    * [ ] determine the PIDs (`sysinfo::Pid`) of running grid servers 
    * [ ] determine the PIDs (`sysinfo::Pid`) running grid workers
  * [ ] implement `stop_server()`
  * [ ] implement `stop_worker()`
  * [ ] Controller: connect to the manager
  * [ ] make it possible to upload libraries
* [ ] implement `app_grid_manager_controller`
* [ ] GUI?
  * Dioxus?
  * [ ] Upload/Exchange Service Libraries
  * [ ] control hwo many workers with which libraries at which times
* [ ] Server: add Tracing: add a member with timestamps of the job per `job_id`
* [ ] `app_worker`: use the service library
  * https://doc.rust-lang.org/std/primitive.pointer.html#common-ways-to-create-raw-pointers
* [ ] `controller_stop_server()`: delay the shutdown so that a response can be send.
* [ ] `client_cpp`: create a sync C++ client
  * [ ] how to check-in/distribute the generated `.cc` and `.h` files?
  * [ ] expose `SyncGridClient`'s methods
    * https://cxx.rs/extern-rust.html#methods
* [ ] expose an async Python Client
  * https://pyo3.rs/v0.13.2/ecosystem/async-await.html#awaiting-a-rust-future-in-python
* [ ] `client_cpp`: create a async C++ client
* Release building:
  * [ ] is currently not triggered by pushing a tag
  * [ ] `release.yml`: `${{ env.GRD_VERSION }}` and `${{ env.RUNNER_OS }}` are empty and thus currently hardcoded
