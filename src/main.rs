use clap::Parser;
pub mod model;
pub mod service;
use crate::model::argument::Argument;
use crate::service::tcp_sender::TcpSender;


fn main() {
    let args = Argument::parse();

    if args.receive {
        println!("Receiver mode enabled. Binding to address: '{}'", &args.addr);
        service::tcp_receiver::TcpReceiver::receive_message(&args.addr).expect("Failed to receive message");
    }


    match &args.send {
        Some(message) => {
            TcpSender::send_message(message.as_str(), args.addr.as_str()).expect("Failed to send message");
        }
        None => {
            panic!("No message provided to send. Use --send <message> to specify a message.");
        }
    }

    println!("Arguments: {:?}", args);

}
