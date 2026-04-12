use std::{io::{Read, Write}, net::TcpStream};

pub struct TcpSender {}

impl TcpSender {
    pub fn send_message<T: AsRef<str>>(message: T, addr: T) -> Result<(), std::io::Error> {
        println!("Sending message: '{}' to address: '{}'", message.as_ref(), addr.as_ref());
        
        let mut stream = TcpStream::connect(addr.as_ref())?;
        stream.write(message.as_ref().as_bytes())?;

        println!("Message sent successfully.");
        Ok(())
    }
} 
