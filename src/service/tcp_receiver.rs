use std::{io::{Read}, net::TcpListener};

pub struct TcpReceiver {}

impl TcpReceiver {
    pub fn receive_message<T: AsRef<str>>(addr: T) -> Result<(), std::io::Error> {
        println!("Listening for messages on address: '{}'", addr.as_ref());
        let listener = TcpListener::bind(addr.as_ref())?;

        for stream_result in listener.incoming() {
            let mut stream = stream_result?;
            let mut buf = [0u8; 1024];

            let n = stream.read(&mut buf)?; // read incoming bytes
            if n > 0 {
                println!("{}", String::from_utf8_lossy(&buf[..n]));
            }
        }

        println!("Completed receiving messages.");

        Ok(())
    }
} 
