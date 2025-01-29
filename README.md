# fyber

[![crates.io](https://img.shields.io/crates/v/fyber.svg)](https://crates.io/crates/fyber)
[![docs.rs](https://docs.rs/fyber/badge.svg)](https://docs.rs/fyber)
[![license](https://img.shields.io/crates/l/fyber.svg)](https://crates.io/crates/fyber)

[![clippy](https://github.com/FL03/fyber/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/fyber/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/fyber/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/fyber/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

`fyber` is a research project considering the interactions between independent computational surfaces. At its core, the resulting protocol bridges together elements of algebraic toplogy with harmonic analysis and music theory to create and manage each of the surfaces.

## Introduction

One day, while doing my daily internet research I had stubled upon, well..., the Wikipidia page on the Neo-Riemannian theory (NRT). This simple event has sparked several outburts over the years, finally manifesting into `fyber` as a harmonic orchestrator for multi-agent/node systems. Initially, I had little understanding of what the theory was even talking about, however, as young coder just learning the fundamentals of computer science and with my partial formal background in engineering and mathematics, I was almost immediately drawn to its dynamics. Never before had I considered the topology or shape of a dynamical system, let alone one of this nature. Over the next couple years, I made several attempts to formalize the protocol to no avail, however, the ideas simply would not leave me alone. It wasn't until I stumbled upon the Wolfram [2, 3] UTM that I was able to finally begin understanding the theory and its implications. The tonnetz and its simplicial components are each individual configuration spaces mapping both an individual agents potentials as well as the collective potential and guiding framework of the system. Meaning the abstraction ends up being a way to describe the potential interactions between the agents and the system as a whole, while further imbuing the system with the ability to have knowledge of its own configuration space and the potential interactions between the other external agents.

## Background

The NRT is a loose collection of inter-related theories describing the behaviours of a specific chord, named the traid, and its transformations. One of the most interesting aspects of the theory is the tonnetz, initially described by Euluer as a two-dimensional lattice that represents the relationships between the chords. It wasn't until later that the tonnetz was able to be successfully generalized as an octohederal, self-looping graph, which is able to represent all the variations of the traid in one single graph.

The theory instantly caught my attention, causing me to dive into it on several occasions over the next couple years. After doing my best to learn the theory, I began to see the potential for using it as a way to harmonize the interactions between various unitized computational surfaces, especially with the rise of agent-based systems and the need for a more robust and flexible way to manage the interactions between them. It took me a while before I stumbled upon my next hint, the Wolfram [2, 3] UTM, where I was able then eventually lead to understand the tonnetz and its simplicial components are each individual configuration spaces mapping both an individual agents potentials as well as the collective potential and guiding framework of the system. Meaning the abstraction ends up being a way to describe the potential interactions between the agents and the system as a whole, while further imbuing the system with the ability to have knowledge of its own configuration space and the potential interactions between the agents and the system as a whole.

## Concepts



## Features

- [ ] agents
- [ ] components: `
- [ ] orchstrator
- [ ] surfaces

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/fyber.git
cd fyber
```

#### _Building the project_

```bash
cargo build --all-features -r -v --workspace
```

#### _Running tests_

```bash
cargo test --all-features -r -v --workspace
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.fyber]
features = []
version = "0.1.0"
```

### Examples

#### _Basic Usage_

```rust
    extern crate fyber;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to {name}", name = fyber);


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
