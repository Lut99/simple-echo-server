# Simple Echo Server 
A simple echo server written in Rust.

Its basic use-case is for testing network connectivity in various scenario's. To this end, the project supports two modes:
- In normal mode, the `simple-echo-server` executable listens for TCP connections and echoes the byte stream
- In HTTP mode, the `simple-echo-server` executable listens for incoming HTTP requests and echoes the bodies of the requests.

In addition, the project includes files to run the executable in [Docker](https://docker.com) and [Kubernetes](https://kubernetes.io).

Note that a precompiled version of the executable is available in the [releases page](https://github.com/Lut99/simple-echo-server/releases/latest) and on [DockerHub]().


## Requirements
To run 'bare metal', make sure you install the following:
- Rust 1.51 or later (see <https://rustup.rs>)
- Cargo (<https://cratio.io>; installed if you use rustup)

To run in Docker in Kubernetes, however, you only need those technologies.


## Compilation
Compilation is only necessary if you don't want to use the prebuilt binary/image.

### Bare metal
First, clone the repository:
```bash
git clone https://github.com/Lut99/simple-echo-server && cd simple-echo-server
```

Next, you can build the project in the (default) TCP mode by using `cargo`:
```bash
cargo build --release
```
which will generate the executable in `./target/release/simple-echo-server`.

Alternatively, to build in HTTP mode, specify the `http` feature:
```bash
cargo build --release --features http
```
which will generate the same executable, `./target/release/simple-echo-server`.


### Docker/Kubernetes
To build the Docker image, run the following the root of the repository:
```bash
docker build -f ./docker/Dockerfile -t simple-echo-server:latest .
```

Possibly, if you have the [Buildx](https://github.com/docker/buildx) plugin installed, you may need to specify the `--load` flag to load the image in your daemon:
```bash
docker build --load -f ./docker/Dockerfile -t simple-echo-server:latest .
```


## Usage
### Bare metal
To run the bare metal version, run the following command:
```bash
./target/release/simple-echo-server
```

Run with the `--help` flag to see other options.

### Docker
To run with Docker, simply run:
```bash
docker run -d --rm --public 80:<PORT> simple-echo-server:latest <PORT>
```

### Kubernetes
Or, on Kubernetes:
```bash
kubectl apply -f ./k8s/deployment.yaml ./k8s/service.yaml
```
(To change the port, change the value of `nodePort` in `./k8s/service.yaml`).

### Calling
Once running, you can connect to it by using `telnet` for the TCP case, or `curl` for the HTTP case. For example:
```bash
# TCP case
telnet localhost <PORT>
```
(You can type and see the echoes, when done, hit Ctrl+], then type 'q' and hit enter)

```bash
# HTTP case, echo
curl -X GET localhost:<PORT> -d "Hello there\n"
```
```bash
# HTTP case, health
curl -X GET localhost:<PORT>/health
```


## Contributing
If you want to contribute anything to this project, welcome! Feel free to checkout the [issues](https://github.com/Lut99/issues) to find anything that needs doing, or create an issue yourself. You can also create pull requests if you're eager and already implemented something.


## License
This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](./LICENSE) file for details.
