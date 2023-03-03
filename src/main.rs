//  MAIN.rs
//    by Lut99
// 
//  Created:
//    03 Mar 2023, 15:17:35
//  Last edited:
//    03 Mar 2023, 15:43:26
//  Auto updated?
//    Yes
// 
//  Description:
//!   The file of the server that accepts incoming connections on the
//!   given port, then echoes back whatever was echoed to it.
// 

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use clap::Parser;
use humanlog::{DebugMode, HumanLogger};
use log::{debug, info, error};
use tokio::net::{TcpListener, TcpStream};


/***** ARGUMENTS *****/
/// Arguments for the server.
#[derive(Debug, Parser)]
struct Arguments {
    /// WHether to enable tracebugging or not
    #[clap(long, global=true, help="If given, enabled more detailled logs.")]
    trace : bool,

    /// The port on which to host the server.
    #[clap(name="port", help="The port on which to host the echo server.")]
    port : u16,
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

    // Attempt to bind the listener
    debug!("Binding to 0.0.0.0:{}...", args.port);
    let listener: TcpListener = match TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), args.port)).await {
        Ok(listener) => listener,
        Err(err)     => { error!("Failed to bind to 0.0.0.0:{}: {}", args.port, err); std::process::exit(1); },
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
