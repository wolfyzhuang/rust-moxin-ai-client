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

source $H