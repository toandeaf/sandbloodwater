use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::RwLock;

use bevy::prelude::Resource;
use lazy_static::lazy_static;

#[derive(Resource)]
pub struct Server(pub HttpServer);

pub struct HttpServer {
    pub listener: TcpListener,
}

impl HttpServer {
    pub fn new(addr: &str) -> Result<HttpServer, Error> {
        let listener = TcpListener::bind(addr)?;

        Ok(HttpServer { listener })
    }
}

lazy_static! {
    static ref HOSTS: RwLock<Vec<TcpStream>> = RwLock::new(Vec::new());
}

pub fn process_connection(stream: TcpStream) {
    if let Ok(mut addresses) = HOSTS.write() {
        addresses.push(stream.try_clone().unwrap());
    }

    let mut buffer = vec![];
    let mut reader = BufReader::new(stream);

    loop {
        if let Ok(bytes_read) = reader.read_until(0x03, &mut buffer) {
            if bytes_read == 0 {
                // End of stream
                break;
            }

            if let Some(pos) = buffer.iter().position(|&x| x == 0x03) {
                let message = &buffer[..pos].to_vec();

                if let Ok(data) = HOSTS.read() {
                    for mut connection in data.iter() {
                        let _write_result = connection.write_all(message);
                    }
                }

                // Remove the processed message from the buffer
                buffer.drain(..=pos);
            }
        }
    }
}
