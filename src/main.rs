use std::io::prelude::*;
use std::{
    error::Error,
    net::{TcpListener, TcpStream},
};

fn accept_conn(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("connected");
    const buf: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(buf.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                accept_conn(_stream)?;
            }
            Err(e) => println!("error: {}", e),
        }
    }
    Ok(())
}
