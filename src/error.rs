use std::io::Write;
use std::net::{Shutdown, TcpStream};

pub struct HttpError {
    pub code: usize,
    pub detail: String,
}

impl HttpError {
    pub fn send_response(self, stream: &mut TcpStream) {
        let response = format!("HTTP/1.1 {} {}\r\n", self.code, self.detail);

        match stream.write(response.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error sending the response: {:?}", e);
            }
        }

        match stream.shutdown(Shutdown::Both) {
            Ok(_) => {}
            Err(e) => {
                println!("error closing the connection: {}", e);
            }
        }
    }
}
