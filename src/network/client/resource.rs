use crate::common::{EventWrapper, EOF};
use bevy::prelude::Resource;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::net::TcpStream;

#[derive(Resource)]
pub struct Client(pub HttpClient);

pub struct HttpClient {
    writer: BufWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}

impl Client {
    pub fn new(addr: &str) -> Result<Client, Error> {
        let connection = TcpStream::connect(addr)?;
        connection.set_nonblocking(true)?;

        let read_connection = connection.try_clone()?;

        Ok(Client(HttpClient {
            reader: BufReader::new(read_connection),
            writer: BufWriter::new(connection),
        }))
    }

    pub fn send_event(&mut self, event: EventWrapper) {
        if let Ok(mut event_bytes) = serde_json::to_vec(&event) {
            event_bytes.push(EOF);

            let bytes_written = self.0.writer.write(&event_bytes).expect("FUCKED THE WRITE");

            if bytes_written == event_bytes.len() {
                self.0.writer.flush().unwrap();
            }

            println!("Sending event: {}", String::from_utf8(event_bytes).unwrap());
        }
    }

    pub fn receive_event(&mut self) -> Option<EventWrapper> {
        let mut buffer = vec![];

        if let Ok(bytes_read) = self.0.reader.read_until(EOF, &mut buffer) {
            println!("BYTES READ {}", bytes_read);

            if bytes_read == 0 {
                // End of stream
                return None;
            }

            // TODO - evaluate if I actually need this given the read_until above.
            if let Some(delimit_position) = buffer.iter().position(|&byte| byte == EOF) {
                let event_wrapper =
                    serde_json::from_slice::<EventWrapper>(&buffer[..delimit_position])
                        .expect("Failed to parse event wrapper.");

                // Remove the processed message from the buffer
                buffer.drain(..=delimit_position);
                return Some(event_wrapper);
            }
        };
        None
    }
}
