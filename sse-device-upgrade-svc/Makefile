.PHONY: build run clean

build:
    cargo build --release --manifest-path sse-device-server/Cargo.toml
    cargo build --release --manifest-path sse-device-client/Cargo.toml
    cargo build --release --manifest-path svckit/Cargo.toml

run-sse-server:
    cargo run --release --manifest-path sse-device-server/Cargo.toml

run-sse-client:
    cargo run --release --manifest-path sse-device-client/Cargo.toml

clean:
    cargo clean
