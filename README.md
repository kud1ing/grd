# grd

A simple computational grid server, using [tonic](https://github.com/hyperium/tonic) ([gRPC](https://grpc.io/) and
[Protocol Buffers 3](https://developers.google.com/protocol-buffers/docs/proto3)).

While the server is written in [Rust](https://www.rust-lang.org), clients and workers can be implemented
in any language:
* bring your own serialization format: the job data and result data are exchanged as raw bytes
* a Python client and worker is provided (using [PyO3](https://pyo3.rs))

## Dependencies

See tonic's [dependencies](https://github.com/hyperium/tonic#dependencies)

## Backlog

* [ ] provide archives of the compilation via GitHub actions
  * https://github.com/BurntSushi/ripgrep/blob/master/.github/workflows/release.yml
  * [ ] fix `create-release`: `Error: Resource not accessible by integration`
  * [ ] fix Upload: `Error: Input required and not supplied: upload_url`
  * [ ] provide the Python clients for different Python versions
* [ ] `client_cpp`: create a sync C++ client
  * [ ] how to check-in/distribute the generated `.cc` and `.h` files?
  * [ ] expose `SyncGridClient`'s methods
    * https://cxx.rs/extern-rust.html#methods
* [ ] expose an async Python Client
  * https://pyo3.rs/v0.13.2/ecosystem/async-await.html#awaiting-a-rust-future-in-python
* [ ] `client_cpp`: create a async C++ client
