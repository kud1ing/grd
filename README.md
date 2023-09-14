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

* make it possible to send commands to the worker via `ResponseToWorkerExchange`:
  * [ ] Stop Worker
    * [ ] implement `controller_stop_worker()`
      * [ ] add the worker client ID to a member `commands_for_workers`
    * [ ] `worker_server_exchange()`: determine `stop_worker` from  `commands_for_workers` 
      * [ ] remove the client information for the worker and from  `commands_for_workers`
  * [ ] Start Worker "<SERVER_ADDRESS> <CLIENT_DESCRIPTION> <SERVICE_ID> <SERVICE_VERSION>"
    * https://doc.rust-lang.org/std/process/struct.Command.html
* [ ] Server status: uses serde-json and send a JSON string
* [ ] `app_worker`: use the service library
  * https://doc.rust-lang.org/std/primitive.pointer.html#common-ways-to-create-raw-pointers
* [ ] `app_worker`: add a signal handler, to capture Ctrl+C
  * https://rust-cli.github.io/book/in-depth/signals.html
  * https://vorner.github.io/2018/06/28/signal-hook.html
* [ ] `shutdown_server()`: shutdown delayed so that a response can be send.
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
