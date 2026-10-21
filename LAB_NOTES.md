# PerfReport Lab 3 Notes

This project migrates Falcon Health Systems’ Windows-specific C++ PerfReport into Rust using the `sysinfo` crate. It reports logical processor count, physical memory use, uptime, first enumerated disk use, and an efficiency score based on the supplied scoring rules.

## Run locally

```powershell
cargo fmt
cargo run
cargo test
cargo build --release
```

## Container

```powershell
docker build -t perfreport .
docker run --rm perfreport
```

## CI

The workflow in `.github/workflows/ci.yml` checks formatting, runs Clippy, builds the release binary, and runs tests. The Docker job depends on the Rust build-and-test job. Push this folder as the **root** of your public GitHub repository so Cargo.toml and Dockerfile are at the repository top level.

## AI assistance disclosure

AI assistance was used to complete the Rust `sysinfo` data collection, add scoring unit tests, and prepare these notes. The student should inspect and explain the code, run the commands on Windows, and capture original evidence screenshots.

## Verification status

Runtime, Docker, and GitHub Actions results are not certified by these notes. Check all three in your own environment and capture the requested screenshots.
