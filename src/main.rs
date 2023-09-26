mod request;
use crate::request::Request;
use std::fs;
use std::io::prelude::*;
use std::sync::Arc;
use std::{env, thread};
use std::{
    error::Error,
    net::{TcpListener, TcpStream},
    str,
};

fn accept_conn(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("connected");
    let mut stream_buffer = [0; 1024];
    stream.read(&mut stream_buffer).unwrap();
    let request = str::from_utf8(&stream_buffer).unwrap();
    let parsed_request = Request::parse(request);
    print!("{} {} ", parsed_request.method, parsed_request.path);
    if parsed_request.path == "/" {
        println!("HTTP/1.1 200 OK");
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .expect("unable to write to stream");
    } else {
        let path_parts = parsed_request.path.split('/').collect::<Vec<&str>>();
        let (status, ctype, body) = if path_parts[1] == "echo" {
            if path_parts.len() > 1 {
                ("200 OK", "text/plain", path_parts[2..].join("/"))
            } else {
                ("200 OK", "text/plain", String::from(""))
            }
        } else if path_parts[1] == "user-agent" {
            (
                "200 OK",
                "text/plain",
                parsed_request.get_header("User-Agent"),
            )
        } else if path_parts[1] == "files" {
            let dir = env::args().nth(2).unwrap_or(".".into());
            let filename = path_parts[1..].join("");
            println!("{dir}/{filename}");
            let contents = fs::read_to_string(format!("{dir}/{filename}"))
                .expect("Should have been able to read the file");

            ("200 Ok", "application/octet-stream", contents)
        } else {
            ("404 Not Found", "text/plain", String::from(""))
        };
        let resp = format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-length:{}\r\n\r\n{}",
            status,
            ctype,
            body.len(),
            body
        );
        println!("{}", status);
        stream.write(resp.as_bytes()).expect("Stream Write Error");
    };

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut handles = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:4221")?;
    println!(
        "Server started at: http://{}",
        listener.local_addr().unwrap()
    );

    // Arc for safe thread data sharing - I have no idea how this works
    let listener = Arc::new(listener);
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let listener = listener.clone(); // Pointer to arc
                handles.push(thread::spawn(move || {
                    accept_conn(&mut _stream).unwrap();
                    drop(listener);
                }));
            }
            Err(e) => println!("error: {}", e),
        }
    }

    // Cleanup
    while handles.len() > 0 {
        let handle = handles.remove(0);
        handle.join().expect("Thread failed...");
    }
    Ok(())
}
