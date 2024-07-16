# Rust Axum API Boilerplate

A boilerplate project for building APIs with [Axum](https://github.com/tokio-rs/axum) in Rust.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running the Project](#running-the-project)
  - [Testing](#testing)


## Features

- [x] Fast and asynchronous API with Axum and Tokio
- [x] Implemented logging.
- [x] Implemented Middlewares
- [x] Implemented auth middleware and cookies
- []  Implemented authentication and authorization schemes
- []  Structured logging with `tracing`
- []  Environment-based configuration with `config` and `dotenv`
- []  Graceful shutdown
- [x] Unit and integration tests
- []  Docker support

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started) (optional, for containerization)
- [Postman](https://www.postman.com/downloads/) (optional, for API testing)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/rust-axum-api-boilerplate.git
    cd rust-axum-api-boilerplate
    ```

2. Install dependencies:

    ```sh
    cargo build
    ```

### Running the Project

1. Start the server:

    ```sh
    cargo run
    ```

2. The API will be available at `http://127.0.0.1:8000`.

### Testing

Run unit and integration tests with:

```sh
cargo test
