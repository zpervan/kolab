# Kolab #

## Setup ##

Install your Rust setup environment - follow this [guide](https://www.rust-lang.org/tools/install).
We use [trunk](https://trunkrs.dev/) for our WASM application bundler and can be installed via Cargo:
```bash
cargo install trunk
```

Additionally, WASM has it's architecture target within Rust which is needed to run the application and can be installed like:
```bash 
 rustup target add wasm32-unknown-unknown
```

## Run ##

### Frontend ###
The frontend is powered by WASM with Trunk. In order to run the frontend application, do the following:
```bash
$ kolab/frontend > trunk serve
```

### Backend ###
```bash
$ kolab/backend > cargo run --release
```