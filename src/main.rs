use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use clap::Parser;

#[derive(Debug)]
struct Message {
    protocol_version: u16,
    nonce: u16,
    text: String,
}

#[derive(clap::Parser, Clone, Copy)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(clap::Subcommand, Clone, Copy)]
enum Mode {
    /// Bind a TCP listener and wait for connections
    Listen { addr: SocketAddr },
    /// Connect to an existing node
    Connect { addr: SocketAddr },
}

fn main() {
    let args = Args::parse();

    let stream = match args.mode {
        Mode::Listen { addr } => TcpListener::bind(addr).unwrap().accept().unwrap().0,
        Mode::Connect { addr } => TcpStream::connect(addr).unwrap(),
    };

    handler(stream);
}

fn handler(mut stream: TcpStream) {
    let mut protocol_version = [0; 2];
    stream.read_exact(&mut protocol_version).unwrap();
    let protocol_version = u16::from_be_bytes(protocol_version);

    let mut data = [0; 4];
    stream.read_exact(&mut data).unwrap();
    let nonce = u16::from_be_bytes(data[0..2].try_into().unwrap());
    let text_len = u16::from_be_bytes(data[2..4].try_into().unwrap()).into();

    let mut text = vec![0; text_len];
    stream.read_exact(&mut text).unwrap();

    let message = Message {
        protocol_version,
        nonce,
        text: String::from_utf8(text).unwrap(),
    };

    println!("{message:?}");
    writeln!(stream, "siemanko").unwrap();
}
