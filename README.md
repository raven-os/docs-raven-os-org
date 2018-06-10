# Raven-OS's Documentaton

Hosts and updates Raven-OS's documentation.

It is based on Rocket.

## Getting Started

These instructions will get your own copy Raven-OS's Documentation up and running on your local machine for development and testing purposes.

See deployment for notes on how to deploy the project on a live system.

### Prerequisites

Use a nightly version of rust
```bash
$ rustup default nightly
```

### Configuration

You can have a look at [Rocket's documentation](https://rocket.rs/guide/configuration/#rockettoml) to see how to configure a `Rocket.toml`. Default settings should be find for debugging purposes.

### Run

```bash
$ cargo run
```

You can tweak the default ip/port with some environment variables (More informations [here](https://rocket.rs/guide/configuration/#rockettoml):

```bash
$ ROCKET_ADDRESS=127.0.0.1 ROCKET_PORT=80 cargo run
```

You may need `sudo` if you want to listen on port `80`.
