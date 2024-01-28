use sandbloodwater::network::{process_connection, HttpServer};
use std::thread;

fn main() {
    let server_res = HttpServer::new("127.0.0.1:7878");

    if let Ok(server) = server_res {
        for stream in server.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "Collecting new connection from: {}",
                        stream.peer_addr().unwrap()
                    );
                    // TODO placeholder for if we need to go single threaded. We'll need to kill streams.
                    // server.connections.push(stream);

                    thread::spawn(|| process_connection(stream));
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }
}
