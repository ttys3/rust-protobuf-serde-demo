# rust-protobuf-serde-demo
## prepare

```shell
cargo new rust-protobuf-serde-demo && cd rust-protobuf-serde-demo

cargo add bytes
cargo add prost

cargo add serde_json
cargo add serde --default-features -F derive -F serde_derive

cargo add --build prost-build
```

## Run

```shell
cargo run
```

## Refs

https://github.com/serde-rs/json

https://docs.rs/prost-serde/latest/prost_serde/

https://docs.rs/prost/latest/prost/

https://docs.rs/prost-build/latest/prost_build/index.html

https://github.com/tokio-rs/prost
