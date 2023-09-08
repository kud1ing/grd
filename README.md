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

* [ ] implement `server::request_status()`
  * https://protobuf.dev/programming-guides/proto3/#maps 
  * [ ] number of jobs
  * [ ] number of results
  * [ ] connected clients
* [ ] add a `app_control`
* [ ] add a `app_worker`
* [ ] `WorkerServerExchangeResponse`: add commands to the worker
  * https://protobuf.dev/programming-guides/enum/
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
