# Changelog
This file keeps track of the changes made to the `simple-echo-server` package.


## 0.2.0 - HTTP variant, Docker/Kubernetes files (10-05-2023)
### Added
- The `http` feature which, when enabled, compiles a variant of the echo server that echoes HTTP requests instead of raw bytes
  - `GET /`, echoes back the body of the incoming HTTP request.
  - `GET /health` always echoes 'OK' in the body of the reply.
- The `--address` option, to change the address to which the server binds.
  - Note this is currently slightly awkward, as the port has to be given here to satisfy `SocketAddr`'s parser, but is then overridden by the normal port argument. Can be fixed, but requires custom address parser.
- A Dockerfile to build it and launch it with Docker.
  - It has also been published to DockerHub: <>.
- Kubernetes Deployment and Service files to launch it with Kubernetes.


## 0.1.0 - Initial release (03-03-2023)
### Added
- A simple echo server that can host on a given port, and echoes-back the bytes passed to it.
