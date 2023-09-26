mod request;
use crate::request::Request;
use std::io::prelude::*;
use std::{
    error::Error,
    net::{TcpListener, TcpStream},
    str,
};
fn accept_conn(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("connected");
    let mut stream_buffer = [0; 1024];
    stream.read(&mut stream_buffer).unwrap();
    let request = str::from_utf8(&stream_buffer).unwrap();
    let parsed_request = Request::parse(request);
    print!("{} {}", parsed_request.method, parsed_request.path);
    if parsed_request.path == "/" {
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .expect("unable to write to stream");
    } else {
        let path_parts = parsed_request.path.split('/').collect::<Vec<&str>>();
        let (status, body) = if path_parts[1] == "echo" {
            if path_parts.len() > 1 {
                ("200 OK", path_parts[2..].join("/"))
            } else {
                ("200 OK", String::from(""))
            }
        } else if path_parts[1] == "user-agent" {
            ("200 OK", parsed_request.getHeader("User-Agent"))
        } else {
            ("404 Not Found", String::from(""))
        };
        let resp = format!(
            "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-length:{}\r\n\r\n{}",
            status,
            body.len(),
            body
        );
        println!(" {}", status);
        stream
            .write(resp.as_bytes())
            .expect("unable to write to stream");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;
    println!(
        "Server started at: http://{}",
        listener.local_addr().unwrap()
    );
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
