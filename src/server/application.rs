use crate::server::context::RequestContext;
use crate::server::error::{ServerError, StdServerError};
use crate::server::parse;
use crate::server::routing::Router;
use crate::server::traits::{Request, RequestMiddleware, Response};

use std::thread;
use std::sync::Arc;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

pub struct ServerConfig {
    pub address: String,
    pub port: usize,
}

pub struct Application<T: Request, R: Response> {
    config: ServerConfig,
    router: Router<T, R>,
    // how do different types of middleware work? box + dyn?
    // middleware: Option<Vec<Arc<dyn RequestMiddleware<T, R> + 'static>>>,
}

impl<T: Request, R: Response> Application<T, R> {
    pub fn new(
        config: ServerConfig,
        router: Router<T, R>,
        // middleware: Option<Vec<Arc<dyn RequestMiddleware<T, R> +'static>>>,
    ) -> Self {
        Self {
            config: config,
            router: router,
            // middleware: middleware,
        }
    }
}

impl<T: Request, R: Response> Application<T, R> {
    fn get_bind(&self) -> String {
        format!("{:}:{:?}", self.config.address, self.config.port)
    }

    fn handle(&self, buffer: String) -> Result<RequestContext<T, R>, ServerError> {
        let mut ctx = RequestContext::<T, R>::new();

        // Parse request
        match parse::parse_into_request(buffer) {
            Ok(val) => {
                ctx.set_request(val);
            }
            Err(e) => return Err(e),
        }

        // Execute middleware pre request
        // match &self.middleware {
        //     Some(val) => {
        //         for middleware in val.iter() {
        //             match middleware.on_request(ctx) {
        //                 Ok(val) => {
        //                     ctx = val;
        //                 }
        //                 Err(e) => return Err(e),
        //             }
        //         }
        //     }
        //     None => {}
        // }

        // Dispatch route handler
        ctx = match self.router.dispatch(ctx) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        // Execute middleware pre response
        // This means errors wont go through this middleware...
        // match &self.middleware {
        //     Some(val) => {
        //         for middleware in val.iter() {
        //             match middleware.on_request(ctx) {
        //                 Ok(val) => {
        //                     ctx = val;
        //                 }
        //                 Err(e) => return Err(e),
        //             }
        //         }
        //     }
        //     None => {}
        // }

        Ok(ctx)
    }

    fn handle_stream(&self, stream: TcpStream) {
        let mut stream = stream;

        let raw = match read_stream(&stream) {
            Ok(val) => val,
            Err(e) => {
                panic!("failed read stream");
            }, 
        };
        
        log::debug!("Buf String: {}", &raw);

        let response = match self.handle(raw) {
            Ok(val) => parse::serialize_into_response(val.get_response()),
            Err(e) => parse::serialize_error_into_response(e),
        };

        send_response(&mut stream, response);
        close_connection(&mut stream);
    }
}

pub fn serve<T: Request + 'static, R: Response + 'static>(application: Application<T, R>) {
    // let application = application;
    let application = Arc::new(application);
    // let application = Arc::<&Application<T, R>>::new(&application);

    let listener = match TcpListener::bind(application.get_bind()) {
        Ok(val) => val,
        Err(e) => {
            println!("Error starting server: {:?}", e);
            panic!("Failed to start server on {:?}", application.get_bind());
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(val) => {
                println!("accepted new connection");
                let arc = Arc::clone(&application);
                thread::spawn(move || {
                    arc.handle_stream(val);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn read_stream(stream: &TcpStream) -> Result<String, ServerError> {
    const BUF_SIZE: usize = 100;
    let mut result = String::new();
    let mut reader = BufReader::new(stream);
    let mut buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];

    // read buffer until less than BUF_SIZE bytes have been read
    loop {
        match reader.read(&mut buffer[..]) {
            Ok(val) => {
                for n in 0..(val) {
                    result.push(char::from(buffer[n]));
                }
                if val != BUF_SIZE {
                    break;
                }
            },
            Err(_) => {
                return Err(StdServerError::BadRequest.to_error());
            },
        }
    }
    Ok(result)
}

fn send_response(stream: &mut TcpStream, response: String) {
    match stream.write(response.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending the response: {:?}", e);
        }
    }
}

fn close_connection(stream: &mut TcpStream) {
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => {}
        Err(e) => {
            println!("error closing the connection: {}", e);
        }
    }
}
