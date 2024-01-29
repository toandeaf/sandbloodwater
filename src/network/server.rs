use crate::common::EventId;
use std::io::{BufRead, BufReader, Error};
use std::net::{TcpListener, TcpStream};

pub struct HttpServer {
    pub listener: TcpListener,
}

impl HttpServer {
    pub fn new(addr: &str) -> Result<HttpServer, Error> {
        let listener = TcpListener::bind(addr)?;

        Ok(HttpServer { listener })
    }
}

pub fn process_connection(stream: TcpStream) {
    let mut buffer = vec![];
    let mut reader = BufReader::new(stream);

    loop {
        match reader.read_until(0x03, &mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // End of stream
                    break;
                }

                if let Some(pos) = buffer.iter().position(|&x| x == 0x03) {
                    let message = &buffer[..pos];

                    if let Ok(event) = serde_json::from_slice::<EventId>(message) {
                        println!("Received event: {}", serde_json::to_string(&event).unwrap());
                    }

                    // Remove the processed message from the buffer
                    buffer.drain(..=pos);
                }
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
