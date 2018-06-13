extern crate byteorder;
extern crate rand;

use std::env;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{thread, time};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rand::prelude::*;

const MAX_MESSAGE_LENGTH: u64 = std::u32::MAX as u64;

struct Peer {
    address: SocketAddr,
    stream: TcpStream,
}

impl Peer {
    fn new(stream: TcpStream) -> Self {
        let address = stream.peer_addr().expect("Unable to get peer address");
        println!("Accepting connection from {}", address);

        Peer { address, stream }
    }

    fn run(&mut self) {
        println!("Waiting for 'hello' message from {}", self.address);

        match self.read_message() {
            Ok(ref m) => {
                if m == "hello" {
                    println!(
                        "Received hello message from {}. Start streaming messages...",
                        self.address
                    )
                } else {
                    eprintln!(
                        "Received unknown message {} from {}. Closing connection.",
                        m, self.address
                    );
                    return;
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to receive message from {}. Closing connection. Error: {}",
                    self.address, e
                );
                return;
            }
        }

        let mut rng = thread_rng();
        let delay = time::Duration::from_secs(1);

        loop {
            let nonce = rng.next_u32();
            let message = format!("nonce: {}", nonce);
            println!("Sending message {} to {}", message, self.address);

            if let Err(e) = self.send_message(&message) {
                eprintln!(
                    "Failed to send message to {}. Closing connection. Error: {}",
                    self.address, e
                );
                return;
            }

            thread::sleep(delay);
        }
    }

    fn read_message(&mut self) -> io::Result<String> {
        let length = self.stream.read_u64::<BigEndian>()?;

        println!(
            "Received length {} of message from {}",
            length, self.address
        );

        if length > MAX_MESSAGE_LENGTH {
            eprintln!(
                "Received message with to large length of {} from {}",
                length, self.address
            );
            return Err(io::ErrorKind::Other.into());
        }

        let mut message = vec![0u8; length as usize];
        self.stream.read_exact(&mut message)?;

        Ok(String::from_utf8_lossy(&message).into())
    }

    fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.stream.write_u64::<BigEndian>(message.len() as u64)?;
        self.stream.write_all(message.as_ref())?;

        Ok(())
    }
}

fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9001".to_string());

    let listener = TcpListener::bind(addr.clone()).expect(&format!("Unable to bind to {}", addr));

    println!("Server running on {}", addr);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(|| {
                    Peer::new(stream).run();
                });
            }
            Err(e) => eprintln!("Unable to accept client connection: {}", e),
        }
    }
}
