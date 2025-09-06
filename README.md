# fyber

[![crates.io](https://img.shields.io/crates/v/fyber?style=for-the-badge&logo=rust)](https://crates.io/crates/fyber)
[![docs.rs](https://img.shields.io/docsrs/fyber?style=for-the-badge&logo=docs.rs)](https://docs.rs/fyber)
[![GitHub License](https://img.shields.io/github/license/FL03/fyber?style=for-the-badge&logo=github)](LICENSE)

***

_**The library is currently in the early stages of development and is still settling in on a feel for the api.**_

Welcome to `fyber`, a crate focused on facilitating communications between independent WebAssembly modules and their constituent components, enabling them to interact seamlessly within a shared environment. The library is designed to be lightweight and efficient, making it suitable for a variety of applications, including serverless computing, edge computing, and microservices architectures.

## Getting Started

For a more detailed guide on getting started, please refer to the [QUICKSTART.md](QUICKSTART.md) file.

### Adding `fyber` to your project

To add `fyber` to your Rust project, include it in your `Cargo.toml` file:

```toml
[dependencies.fyber]
version = "0.0.x"
features = [
    "default",
]
```

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/fyber.git
```

Then, change into the directory:

```bash
cd fyber
```

```bash
cargo build --all-features --workspace
```

#### _Run an example_

```bash
cargo run -f F --example {actor}
```

### Examples

You can find various examples in the [examples](fyber/examples) directory. Each example demonstrates different features and use cases of the `fyber` library.

## Contributing

Contributions are welcome! For more information visit the [CONTRIBUTING.md](CONTRIBUTING.md) file.
