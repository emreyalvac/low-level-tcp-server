use std::net::{TcpListener, TcpStream};
use std::fmt::{Debug};
use std::io::{Write, Read};
use std::path::Path;
use std::collections::HashMap;

// Request Struct
#[derive(Debug)]
struct Request<'a> {
    uri: &'a str,
    http_version: &'a str,
    method: &'a str,
}

#[derive(Debug)]
enum RequestError {
    UnsupportedMethod,
    UnsupportedHttpVersion,
}

fn main() {
    let ip = "127.0.0.1:5002";
    let listener = TcpListener::bind(ip).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(str) => {
                std::thread::spawn(|| {
                    handle_connection(str).unwrap()
                });
            }
            Err(err) => println!("Connection Failed {:?}", err)
        }
    }
}

fn s0() -> &'static str {
    return "route 1";
}

fn s1() -> &'static str {
    return "route 2";
}

// Handle Connection
fn handle_connection(mut stream: TcpStream) -> Result<(), bool> {
    // Read 4096 bytes from connection
    let mut buffer = [0u8; 4096];

    // Writes stream to buffer
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap();
    let parsed = parse_request(request_line);

    let mut routes = HashMap::<String, &dyn Fn() -> &'static str>::new();
    routes.insert(String::from("/"), &s0);
    routes.insert(String::from("/route2"), &s1);

    match parsed {
        Ok(request) => {
            for route in routes.iter() {
                if route.0.as_str() == request.uri {
                    let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", route.1());
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
        Err(e) => println!("{:?}", e)
    }

    Ok(())
}

// Parse Request
fn parse_request(request: &str) -> Result<Request, RequestError> {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("Method not specificed");
    if method != "GET" {
        // Unsupported
        return Err(RequestError::UnsupportedMethod);
    }

    // URI
    let uri = Path::new(parts.next().unwrap_or("URI not specified"));
    let norm_uri = uri.to_str().expect("Invalid Encode");

    let http_version = parts.next().expect("Http version not specified");
    if http_version != "HTTP/1.1" {
        return Err(RequestError::UnsupportedHttpVersion);
    }

    Ok(Request { method, http_version, uri: norm_uri })
}
