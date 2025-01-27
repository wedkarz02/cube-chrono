## Table of Contents

* [Requirements](#requirements)
* [Download](#download)
* [Build](#build)
* [Getting Started](#getting-started)
  * [Environment](#environment)
  * [Compose](#compose)
* [Usage](#usage)
* [License](#license)

## Requirements

→ [Rust](https://www.rust-lang.org/)\
→ [Cargo](https://doc.rust-lang.org/cargo/)\
→ [MongoDB](https://www.mongodb.com/)

Alternatively you can run MongoDB in [Docker](https://www.docker.com/) by building it from the `compose.yml` file.
More info in the [Getting Started](#getting-started) section.

## Download

Download the source code using the ```git clone``` command:

```bash
$ git clone https://github.com/wedkarz02/cube-chrono.git
```

Or use the *Download ZIP* option from the Github repository [page](https://github.com/wedkarz02/cube-chrono.git).

## Build

Build the application with `cargo`:

```bash
$ cargo build --release
```

## Getting Started

### Environment

The API is configured with those environment variables:

|                            | Description                                        |
|----------------------------|----------------------------------------------------|
| MONGO_INITDB_ROOT_USERNAME | Root username used when initializing the database. |
| MONGO_INITDB_ROOT_PASSWORD | Root password used when initializing the database. |
| MONGO_INITDB_DATABASE      | Database name.                                     |
| MONGO_URI                  | Database URI (must include credentials).           |
| BACKEND_PORT               | HTTP port of the API.                              |
| JWT_ACCESS_SECRET          | Secret value for access tokens.                    |
| JWT_REFRESH_SECRET         | Secret value for refresh tokens.                   |
| SUPERUSER_PASSWORD         | Initial admin's password.                          |

`MONGO_INITDB_ROOT_USERNAME` and `MONGO_INITDB_ROOT_PASSWORD` are only used by Docker, everything else is mandatory.

To make the setup easier you can copy the environment template file:

```bash
$ cp .example.env .env
```

and fill the `.env` file with your configuration.

### Compose

If you don't have MongoDB installed you can build it with `compose.yml` file using Docker:

```bash
$ docker compose up -d
```

which will download the MongoDB image and build a container with credentials from `.env` file.

To check if the container is running use:

```bash
$ docker ps -a
```

and to stop it:

```bash
$ docker compose down
```

### Usage

When in `cube-chrono/backend`, run the API application with `cargo`:

```bash
$ cargo run --release
```

When the API is live you can use any endpoints specified in the [API Reference](https://github.com/wedkarz02/cube-chrono/blob/main/doc/api-documentation.md) document.

## License

If not directly stated otherwise, everything in this project is licensed under the MIT License. See the [LICENSE](https://github.com/wedkarz02/cube-chrono/blob/main/LICENSE) file for more info.
