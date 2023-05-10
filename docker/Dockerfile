# DOCKERFILE for the `simple-echo-server`
#   by Lut99
# 
# A Dockerfile for the simple-echo-server project.
# 
# Run as:
# ```bash
# docker build -f ./docker/Dockerfile .
# ```
# from the repository's root folder.
# 
# To run the HTTP variant, run with the 'http' build argument instead.
# 


### BUILD IMAGE ###
# We build in the rust container first
FROM rust:1-alpine3.17 AS build

# Define the build method
ARG mode=tcp

# Define build dependencies
RUN apk add --no-cache musl-dev

# Copy the source files
RUN mkdir -p /build
COPY ./src /build/src/
COPY ./Cargo.toml /build/
COPY ./Cargo.lock /build/

# Run the build command
WORKDIR /build
RUN cargo build --release $(if [ "$mode" = "http" ]; then echo "--features"; fi) $(if [ "$mode" = "http" ]; then echo "http"; fi) \
 && mv ./target/release/simple-echo-server /simple-echo-server \
 && rm -rf target



### RUN IMAGE ###
# Start from alpine linux
FROM alpine:3.17

# Install runtime dependencies
RUN apk add --no-cache libc6-compat gcompat libgcc

# Copy the executable from the build step
COPY --from=build /simple-echo-server /simple-echo-server
RUN chmod +x /simple-echo-server

# Set the entrypoint
ENTRYPOINT [ "/simple-echo-server" ]
CMD [ "80" ]
