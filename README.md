# grd

A minimal stream processing grid software using [gRPC](https://grpc.io/) and
[Protocol Buffers 3](https://developers.google.com/protocol-buffers/docs/proto3).

While the client and server are written in [Rust](https://www.rust-lang.org), clients and workers can be implemented
in any language:
* a C API is provided
* bring your own serialization format: the job data and result data are raw bytes

## Backlog

* [ ] create C++ client
* [ ] expose an async Python Client?
  * https://pyo3.rs/v0.13.2/ecosystem/async-await.html#awaiting-a-rust-future-in-python
* [ ] provide binaries via GitHub actions:
  * https://alican.codes/rust-github-actions
  * https://github.com/BurntSushi/ripgrep/blob/master/.github/workflows/release.yml
