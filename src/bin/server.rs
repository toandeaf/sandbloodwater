use bevy::prelude::Event;
use serde::{Deserialize, Serialize};
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Event)]
pub enum EventId {
    Test(String),
    More(String),
}

struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    fn new(addr: &str) -> Result<HttpServer, Error> {
        let listener = TcpListener::bind(addr)?;

        Ok(HttpServer { listener })
    }
}

fn handle_client(mut stream: TcpStream) {
    let address = match stream.peer_addr() {
        Ok(value) => value.ip().to_string() + ":" + &*value.port().to_string(),
        Err(_) => String::from("BACKUP"),
    };

    let mut buffer = [0; 512];
    let msg = b"ack";

    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size > 0 {
                let data = &buffer[..size];

                let event_content_res = serde_json::from_slice::<EventId>(data);

                if let Ok(event_data) = event_content_res {
                    // TODO placeholder for when we come to integrate
                    // consume_and_apply_event(event_data);
                    match event_data {
                        EventId::Test(data) => println!("This data is test {}", data),
                        EventId::More(data) => println!("This data is more {}", data),
                    }
                }

                let write_result = stream.write(msg);

                write_result.is_ok()
            } else {
                false
            }
        }
        Err(_) => {
            eprintln!("Client {} has disconnected.", address);
            let shutdown_res = stream.shutdown(std::net::Shutdown::Both);

            shutdown_res.is_err()
        }
    } {}
}

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

                    thread::spawn(|| handle_client(stream));
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }
}
