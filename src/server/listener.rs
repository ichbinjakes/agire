// use crate::error;
// use crate::http11;

// use std::io::{BufRead, BufReader, Read, Write};
// use std::net::{Shutdown, TcpListener, TcpStream};

// pub struct Server {
//     address: String,
//     port: usize,
// }

// impl Server {
//     fn get_bind(&self) -> String {
//         format!("{:?}}:{:?}", self.address, self.port)
//     }

//     pub fn serve(&self) {
//         let listener = TcpListener::bind(self.get_bind()).unwrap();

//         for stream in listener.incoming() {
//             match stream {
//                 Ok(mut _stream) => {
//                     println!("accepted new connection");

//                     // Process incoming data
//                     let mut buf = String::new();
//                     let mut stream_reader = BufReader::new(&_stream);

//                     match stream_reader.read_line(&mut buf) {
//                         Ok(_) => match http11::parse_request_line(&buf) {
//                             Some(val) => {}
//                             None => {}
//                         },
//                         Err(_) => {}
//                     }                   
//                 }
//                 Err(e) => {
//                     println!("error: {}", e);
//                 }
//             }
//         }
//     }

//     // pub fn start(&self) {
//     //     let listener = TcpListener::bind(self.get_bind()).unwrap();

//     //     for stream in listener.incoming() {
//     //         match stream {
//     //             Ok(mut _stream) => {
//     //                 println!("accepted new connection");

//     //                 // Process incoming data
//     //                 let mut buf = String::new();
//     //                 let mut stream_reader = BufReader::new(&_stream);

//     //                 match stream_reader.read_line(&mut buf) {
//     //                     Ok(_) => match http11::parse_request_line(&buf) {
//     //                         Some(val) => {
//     //                             println!("Method: {}, URI: {}, Version: {}", val.0, val.1, val.2);
//     //                             if val.1 != "/" {
//     //                                 error::HttpError {
//     //                                     code: 404,
//     //                                     detail: String::from("Not Found"),
//     //                                 }
//     //                                 .send_response(&mut _stream);
//     //                             } else {
//     //                                 match _stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
//     //                                     Ok(_) => {
//     //                                         println!("Sent 200 response");
//     //                                     }
//     //                                     Err(_) => {
//     //                                         error::HttpError {
//     //                                             code: 500,
//     //                                             detail: String::from("Internal Server Error"),
//     //                                         }
//     //                                         .send_response(&mut _stream);
//     //                                     }
//     //                                 }
//     //                                 match _stream.shutdown(Shutdown::Both) {
//     //                                     Ok(_) => {}
//     //                                     Err(e) => {
//     //                                         println!("error closing the connection: {}", e);
//     //                                     }
//     //                                 }
//     //                             }
//     //                         }
//     //                         None => {
//     //                             error::HttpError {
//     //                                 code: 400,
//     //                                 detail: String::from("Bad Request"),
//     //                             }
//     //                             .send_response(&mut _stream);
//     //                         }
//     //                     },
//     //                     Err(_) => {
//     //                         error::HttpError {
//     //                             code: 500,
//     //                             detail: String::from("Internal Server Error"),
//     //                         }
//     //                         .send_response(&mut _stream);
//     //                     }
//     //                 }                   
//     //             }
//     //             Err(e) => {
//     //                 println!("error: {}", e);
//     //             }
//     //         }
//     //     }
//     // }
// }
