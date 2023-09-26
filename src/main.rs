mod request;
use crate::request::Request;
use std::fs;
use std::io::prelude::*;
use std::{env, thread};
use std::{
    error::Error,
    net::{TcpListener, TcpStream},
    str,
};

fn accept_conn(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("connected");
    let mut stream_buffer = [0; 2048];
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
        let path = &parsed_request.path;
        let (status, ctype, body) = if path.starts_with("/echo/") {
            ("200 OK", "text/plain", path[6..].to_string())
        } else if path.starts_with("/user-agent") {
            (
                "200 OK",
                "text/plain",
                parsed_request.get_header("User-Agent"),
            )
        } else if path.starts_with("/files/") {
            let dir = env::args().nth(2).unwrap_or(".".into());
            let filename = path[7..].to_string();
            if parsed_request.method == "POST" {
                match fs::write(format!("{dir}{filename}"), parsed_request.body) {
                    Ok(()) => ("201 Created", "application/octet-stream", "".to_string()),
                    Err(err) => (
                        "500 Server Error",
                        "application/octet-stream",
                        err.to_string(),
                    ),
                }
            } else {
                let contents = fs::read_to_string(format!("{dir}{filename}"))
                    .expect("Should have been able to read the file")
                    .trim()
                    .to_string();
                println!("{contents},{}", contents.len());
                ("200 Ok", "application/octet-stream", contents)
            }
        } else {
            ("404 Not Found", "text/plain", String::from(""))
        };
        let resp = format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length:{}\r\n\r\n{}",
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
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handles.push(thread::spawn(move || {
                    accept_conn(&mut _stream).unwrap_or_else(|e| {
                        eprintln!("error: {}", e);
                    });
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
