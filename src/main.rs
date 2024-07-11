use std::io::Write;
use std::net::{Shutdown, TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                match _stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
                    Ok(_) => {
                        println!("Sent 200 response");
                    }
                    Err(e) => {
                        println!("send error: {}", e);
                    }
                }
                match _stream.shutdown(Shutdown::Both) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("error closing the connection: {}", e);
                    },
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
