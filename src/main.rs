use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use clap::Parser;
use std::error::Error;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};

fn bincode_config() -> impl bincode::config::Config {
    use bincode::config::*;
    // Use fixed int encoding so that the protocol is easy to implement in other
    // clients and doesn't require implementing the specific variable encoding
    // algorithm used by bincode
    Configuration::<BigEndian, Fixint, NoLimit>::default()
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
struct Message {
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

    match args.mode {
        Mode::Listen { addr } => {
            let stream = TcpListener::bind(addr).unwrap().accept().unwrap().0;
            recv_message(stream).unwrap();
        },
        Mode::Connect { addr } => {
            let stream = TcpStream::connect(addr).unwrap();
            send_message(stream).unwrap();
        },
    }
}

fn recv_message(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // TODO: Check protocol version
    let _protocol_version = stream.read_u16::<BigEndian>()?;
    let message: Message = bincode::decode_from_std_read(&mut stream, bincode_config())?;
    println!("{message:?}");

    Ok(())
}

fn send_message(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let message = Message {
        nonce: u16::MAX,
        text: "siemanko".into(),
    };
    stream.write_u16::<BigEndian>(1)?;

    let bytes = bincode::encode_to_vec(&message, bincode_config())?;
    println!("encoded message: {bytes:x?}");

    // ```
    // let bytes_written = bincode::encode_into_std_write(message, &mut stream, bincode_config())?;
    // println!("wrote {bytes_written} bytes");
    // ```

    stream.write_all(&bytes)?;
    println!("wrote {} bytes", bytes.len());

    Ok(())
}
