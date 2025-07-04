use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// Entry Point: Launches an HTTP server on localhost: 3000
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Address to bind the TCP Listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🌐 Starting HTTP server at http://{addr}");

    // Bind to the address using a TCP Listener
    let listener = TcpListener::bind(addr).await?;

    // Loop to continuously accept incoming connections
    loop {
        // Accept a connection from a client
        let (stream, client_address) = listener.accept().await?;
        println!("🔌 Connection accepted from: {client_address}");

        // Wrap the raw stream in TokioIo to make it compatible with Hyper
        let io = TokioIo::new(stream);

        // Spawn a new async task for concurrent handling
        tokio::task::spawn(async move {
            // Bind the connection to the HTTP service
            if let Err(err) = http1::Builder::new()
            .serve_connection(io, service_fn(hello_handler)) // 'service_fn' converts fn 'hello_handler' in a "Service"     
            .await {
                eprintln!("❌ Error serving connection: {:?}", err);
            }
        });
    }

}

/// Handler function that generates a basic HTTP response
async fn hello_handler(_req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let res_txt = "👋 Hello World From Hyper & Tokio!";
    Ok(Response::new(Full::new(Bytes::from(res_txt))))
}

/*
📘 Explanation:

1. **main function**:
   - Starts a TCP listener on `127.0.0.1:3000`.
   - Uses `tokio::net::TcpListener` for async I/O.
   - Spawns tasks for concurrent request handling using `tokio::spawn`.

2. **TokioIo::new(stream)**:
   - Converts a Tokio TCP stream into a form Hyper can work with.

3. **http1::Builder::new().serve_connection(...)**:
   - Sets up the HTTP/1.1 connection using Hyper.
   - `service_fn(hello_handler)` converts a regular async function into a `Service` compatible with Hyper.

4. **hello_handler**:
   - Always returns a static `Hello, World` message as the response.
   - The body uses `Full<Bytes>` for a simple full-body HTTP response
*/