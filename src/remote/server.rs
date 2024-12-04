use bevy::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct RemoteServerPlugin;

impl Plugin for RemoteServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_remote_server);
    }
}

#[tokio::main]
async fn initialize_remote_server() {
    // TODO - Initialize and check that assets are loaded.

    // Initialize TCP listener
    let listener = TcpListener::bind("0.0.0.0:8040").expect("Failed to bind to port 8040");
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    // Initialize the thread pool
    let pool = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    for stream in listener.incoming().flatten() {
        pool.spawn(async move {
            handle_stream(stream);
        });
    }
}

fn handle_stream(mut stream: TcpStream) {
    // Basic debug
    let mut buffer = [0; 1024];
    let read_no = stream.read(&mut buffer).unwrap();
    let value = String::from_utf8_lossy(&buffer);
    println!("Received: {} bytes, value: {}", read_no, value);

    // TODO - Read the stream values to a buffer, then attempt to deserialize and type.
}
