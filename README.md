# Simple Echo Server in Rust
This is a simple echo server written in Rust using the tokio library.

## Requirements
- Rust 1.51 or later
- Cargo


## Usage
1. Clone the repository:

```sh

git clone https://github.com/Lut99/simple-echo-server
```

2. Change into the project directory:

```sh

cd simple-echo-server-rust
```

3. Build the project:

```sh

cargo build --release
```

4. Run the server:

```sh

cargo run --release
```

The server will start listening on `localhost:8080`.

Test the server:

5. Open a new terminal window and run:

```sh

curl -X POST -H "Content-Type: application/json" -d '{"message":"hello world"}' http://localhost:8080/echo
```

You should see the response **{"message":"hello world"}**.

 ## License
This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](./LICENSE) file for details.
