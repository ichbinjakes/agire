pub mod abnf;
pub mod error;
pub mod http11;
pub mod server;
pub mod types;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

fn main() {
    let svr = server::Server {};
    svr.start();

    // println!("Logs from your program will appear here!");

    // let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // for stream in listener.incoming() {
    //     match stream {
    //         // return TcpStream
    //         Ok(mut _stream) => {
    //             println!("accepted new connection");

    //             // Process incoming data
    //             let mut buf = String::new();
    //             let mut stream_reader = BufReader::new(&_stream);
    //             match stream_reader.read_line(&mut buf) {
    //                 Ok(val) => {
    //                     println!("Read {} bytes from tcp stream", val);
    //                     //println!("Read content {:?}", buf);
    //                     let request = parse_request(&buf);
    //                     let request_line = match parse_request_line(request) {
    //                         Ok(val) => val,
    //                         Err(e) => {}
    //                     };
    //                     println!("Request Line: {}", request[0]);
    //                     println!("Headers     : {}", request[1]);
    //                     println!("Body        : {}", request[2]);
    //                 }
    //                 Err(val) => {
    //                     println!("Error reading strem: {:?}", val);
    //                 }
    //             }

    //             // Send the response
    //             match _stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
    //                 Ok(val) => {
    //                     println!("Sent 200 response");
    //                 }
    //                 Err(e) => {
    //                     println!("send error: {}", e);
    //                 }
    //             }

    //             // Shutdown the connection
    //             match _stream.shutdown(Shutdown::Both) {
    //                 Ok(_) => {}
    //                 Err(e) => {
    //                     println!("error closing the connection: {}", e);
    //                 }
    //             }

    //             // End
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}

// fn process_request(stream: &mut TcpStream) -> Result<(), HttpError> {}

// fn parse_request_line(line: &str) -> Result<RequestLine, HttpError> {
//     let mut iterator = line.split(' ').into_iter();
//     let method = match HttpMethod::from_str(iterator.next().unwrap()) {
//         Ok(val) => val,
//         Err(e) => return e,
//     };
//     let path = iterator.next().unwrap();
//     let version = match HttpVersion::from_str(iterator.next().unwrap()) {
//         Ok(val) => val,
//         Err(e) => return e,
//     };
//     Ok(RequestLine {
//         method: method,
//         path: String::from(path),
//         version: version,
//     })
// }
