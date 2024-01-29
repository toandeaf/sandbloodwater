use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::common::EventId;

pub struct HttpServer {
    pub listener: TcpListener,
}

impl HttpServer {
    pub fn new(addr: &str) -> Result<HttpServer, Error> {
        let listener = TcpListener::bind(addr)?;

        Ok(HttpServer { listener })
    }
}

pub fn process_connection(mut stream: TcpStream) {
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

                // TODO do I just fuckin implement the broadcast and go for a straight memcopy of the vec data? lol
                match event_content_res {
                    Ok(_event_data) => {
                        // TODO placeholder for when we come to integrate
                        // consume_and_apply_event(event_data);
                    }
                    Err(error) => {
                        eprintln!("Error deserialising event message: {}", error);
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
