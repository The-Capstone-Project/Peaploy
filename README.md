# Peaploy

Peaploy is a Rust-based project that uses the `tree-sitter` library to parse code and try to  analyze import, system, and build dependencies.

## Project Structure

```txt
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── model.pkl
├── README.md
└── src/
    └── main.rs
```

## Prerequisites

- Rust (latest stable version)
- Python 3.x
- `pip` (Python package installer)

## Setup

### Rust Setup

1. **Install Rust**: Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust.

2. **Clone the Repository**:

    ```sh
    git clone <repository-url>
    cd peaploy
    ```

3. **Build the Project**:

    ```sh
    cargo build
    ```

4. **Run the Project**:

    ```sh
    cargo run
    ```

## Usage

### Rust Application

Peaploy makes use of the `Walkdir` and standard libraries in rust to read all the files from a project folder, to analyse its dependecies. To run the application, use:

```sh
cargo run
```

## License

This project is licensed under the MIT License.
