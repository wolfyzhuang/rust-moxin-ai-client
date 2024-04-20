# Rust Moxin: An AI LLM client built atop [Robius](https://github.com/project-robius)

Rust Moxin is an AI LLM client showcasing the functionalities of Robius, a multi-platform application development framework. The project is still in the early stages and not fully functional yet.

The below table shows the host systems that currently support building Robrix for different target platforms.
| Host OS | Target Platform | Builds? | Runs? |
| ------- | --------------- | ------- | ----- |
| macOS | macOS | ✅ | ✅ |
| Linux | ubuntu(x86_64-unknown-linux-gnu) | ✅ | ? |

## Build and Run Instructions

1. [Install Rust](https://www.rust-lang.org/tools/install) first.

2. Install the required WasmEdge WASM runtime:

```sh
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- --plugins wasi_nn-ggml

source $HOME/.wasmedge/env
```

### macOS Users

For desktop users on macOS, you simply have to run:

```sh
cd ~/rust-moxin-ai-client
cargo run
```

### Linux Users

On Linux, ensure the following dependencies are installed (considering Ubuntu as an example):

```sh
sudo apt-get update
# openssl
sudo apt-get install libssl-dev pkg-config
# libclang for bindgen
sudo apt-get install llvm clang libclang-dev
# binfmt
sudo apt install binfmt-support
# Xcursor、X11、asound and pulse
sudo apt-get install libxcursor-dev libx11-dev libasound2-dev libpulse-dev
```

Then, run the following commands:

```sh
cd ~/rust-moxin-ai-client
cargo run
```