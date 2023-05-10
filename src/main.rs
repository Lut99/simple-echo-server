//  MAIN.rs
//    by Lut99
// 
//  Created:
//    03 Mar 2023, 15:17:35
//  Last edited:
//    10 May 2023, 10:46:03
//  Auto updated?
//    Yes
// 
//  Description:
//!   The file of the server that accepts incoming connections on the
//!   given port, then echoes back whatever was echoed to it.
// 

use std::net::SocketAddr;

use clap::Parser;
use humanlog::{DebugMode, HumanLogger};
use log::{debug, info};


/***** ARGUMENTS *****/
/// Arguments for the server.
#[derive(Debug, Parser)]
struct Arguments {
    /// The port on which to host the server.
    #[clap(name="port", help="The port on which to host the echo server.")]
    port : u16,

    /// Whether to enable tracebugging or not
    #[clap(long, global=true, help="If given, enabled more detailled logs.")]
    trace   : bool,
    /// The address on which to serve.
    #[clap(long, global=true, default_value="0.0.0.0:0", help="The address on which to bind the server. Note that the port-part will be overridden by the 'PORT' value.")]
    address : SocketAddr,
}





/***** LIBRARY *****/
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Parse the arguments
    let args: Arguments = Arguments::parse();

    // Setup the logger
    if let Err(err) = HumanLogger::terminal(if args.trace { DebugMode::Full } else { DebugMode::Debug }).init() {
        eprintln!("WARNING: Failed to setup logger: {} (no logging enabled for this session)", err);
    }
    info!("Initializing Simple Echo Server v{}...", env!("CARGO_PKG_VERSION"));

    // Prepare a single socketaddr with the port included
    let mut addr: SocketAddr = args.address;
    addr.set_port(args.port);

    // Match on the compiled mode
    #[cfg(not(feature = "http"))]
    {
        use log::error;
        use tokio::net::{TcpListener, TcpStream};


        // Attempt to bind the listener
        debug!("Binding to 0.0.0.0:{}...", args.port);
        let listener: TcpListener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(err)     => { error!("Failed to bind to {}: {}", args.address, err); std::process::exit(1); },
        };

        // Listen
        loop {
            debug!("Listening for new connections...");
            let (stream, ip): (TcpStream, SocketAddr) = match listener.accept().await {
                Ok(res)  => res,
                Err(err) => { error!("Failed to accept new client: {}", err); continue; },
            };

            // Handle it on a separate task
            tokio::spawn(async move {
                info!("{}: Accepted new connection", ip);

                // Read from the client as long as they want
                loop {
                    // Wait asynchronously for the socket to become readable
                    if let Err(err) = stream.readable().await { error!("{}: Failed to wait for client stream to become readable: {} (quitting connection)", ip, err); break; }

                    // Read the next chunk
                    let mut bytes: [u8; 4096] = [0; 4096];
                    let n_bytes: usize = match stream.try_read(&mut bytes) {
                        Ok(n_bytes) => n_bytes,
                        Err(err)    => if err.kind() != std::io::ErrorKind::WouldBlock { error!("{}: Failed to read from client stream: {} (quitting connection)", ip, err); break; } else { continue; },
                    };

                    // Quit if there is nothing left to read
                    if n_bytes == 0 {
                        info!("{}: Client disconnected, closing connection", ip);
                        break;
                    }

                    // Otherwise, write
                    debug!("{}: Received {} bytes", ip, n_bytes);
                    if let Err(err) = stream.try_write(&bytes[..n_bytes]) { error!("{}: Failed to write to client stream: {} (quitting connection)", ip, err); break; }
                }

                // Done
            });
        }
    }



    #[cfg(feature = "http")]
    {
        use warp::Filter as _;
        use warp::hyper::Body;
        use warp::hyper::body::Bytes;
        use warp::reject::Rejection;
        use warp::reply::Response;


        /// Handles an echo-request.
        /// 
        /// # Arguments
        /// - `remote`: The address of the remote host if it is known.
        /// - `body`: The raw body, passed as bytes.
        /// 
        /// # Returns
        /// The same body as given, wrapped in a proper [`Response`].
        /// 
        /// # Errors
        /// This function never errors.
        async fn echo_handler(remote: Option<SocketAddr>, body: Bytes) -> Result<Response, Rejection> {
            info!("Accepted echo-request from {}", if let Some(addr) = remote { format!("{addr}") } else { "<unknown>".into() });
            Ok(Response::new(Body::from(body)))
        }

        /// Handles a health-request.
        /// 
        /// # Arguments
        /// - `remote`: The address of the remote host if it is known.
        /// 
        /// # Returns
        /// A new [`Response`] with 'OK' in the body.
        /// 
        /// # Errors
        /// This function never errors.
        async fn health_handler(remote: Option<SocketAddr>) -> Result<Response, Rejection> {
            info!("Accepted heatlh-request from {}", if let Some(addr) = remote { format!("{addr}") } else { "<unknown>".into() });
            Ok(Response::new(Body::from("OK")))
        }


        // Prepare a warp path to run the echo server
        debug!("Preparing warp server...");
        let echo = warp::get()
            .and(warp::path::end())
            .and(warp::filters::addr::remote())
            .and(warp::body::bytes())
            .and_then(echo_handler);

        // Prepare another to run the health server
        let health = warp::get()
            .and(warp::path("health"))
            .and(warp::path::end())
            .and(warp::filters::addr::remote())
            .and_then(health_handler);

        // Join them in one filter and serve that
        debug!("Waiting for connections...");
        warp::serve(echo.or(health)).run(addr).await;
    }
}
