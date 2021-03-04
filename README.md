# RC Assignment

RC Assignment for producing endpoint telemetry to compare with an EDR agent. Designed for MacOS and Linux (UNIX)

## Getting Started

You will need both Rust and Cargo. Please visit [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) for information how to get started and install on your system.

### Prerequisites

All pre-requisite external rates will be handled by Cargo. External crates used are:

```
clap = {version = "~2.27.0", features = ["yaml"]}
users = "0.11"
chrono = "0.4"
local_ipaddress = "0.1.3"
```

### Setting up

- Clone repository to local machine. [Cloning a repository](https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/cloning-a-repository)

- Change in to directory 'edr_test'

- Run command 'cargo build' 

## Running

Can run using cargo:
```
cargo run -- <record_file> <ip_address> <dst_port> <src_port> <test_file> <process> [OPTIONS]
```

or calling
```
./target/debug/edr_test <record_file> <ip_address> <dst_port> <src_port> <test_file> <process> [OPTIONS]
