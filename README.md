# Rust API template

Project template for a Rust API server. This template is designed to be a starting point for new projects. It includes a basic API server, logging, configuration, and a few other features.

## Pre-requisites

On Linux systems, you may need to install the following dependencies:

```bash
apt install libssl-dev
```

Whenever submodules are added, updated or removed, the following command should be run:

```bash
git submodule update --init
```

Currently there are no submodules.

## Server binaries

Build production binaries

```bash
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build --release --bin api -j $(nproc)
# RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort --target x86_64-apple-darwin --release
cargo build --release -j $(nproc)
```

Compress binaries using [UPX](https://github.com/upx/upx), if needed

```bash
upx --best --lzma target/release/min-sized-rust
```

## Containers

Sometimes it's advantageous to deploy Rust into containers
(e.g. [Docker](https://www.docker.com/)). There are several great existing resources to help
create minimum sized container images that run Rust binaries.

- [Official `rust:alpine` image](https://hub.docker.com/_/rust)
- [mini-docker-rust](https://github.com/kpcyrd/mini-docker-rust)
- [muslrust](https://github.com/clux/muslrust)
- [docker-slim](https://github.com/docker-slim/docker-slim) - Minify Docker images

### Build container

```bash
docker compose build --remove-orphans
```

### Run container (detached)

```bash
docker compose up -d
```

### Run and build container attached

```bash
docker compose up --build --remove-orphans
```

### Run server

```bash
cargo run --bin rust-api-template
```

## Live development

```bash
cargo doc --open
cargo watch -p api -x "run --bin rust-api-template"
```

### Generate documentation

```bash
cargo doc --no-deps --open
```

### Generate SHA256

```bash
echo -n "your-string" | openssl dgst -sha256
```

## Working with Upstream

```bash
git fetch upstream
git checkout main  # or the branch you're working on
git merge upstream/main
```

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## TODO

- [ ] Add DOCS and examples for everything
- [ ] dynamic handling of different log levels
- [ ] Add database library
- [ ] Add tests
- [ ] Add benchmarking
- [ ] Add CI/CD
