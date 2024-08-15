use crate::server::traits::{RequestMiddleware,Request,Response};
use crate::server::context::RequestContext;
use crate::server::error::ServerError;
use crate::server::routing::Router;
use crate::server::parse;

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
    middleware: Option<Vec<Box<dyn RequestMiddleware<T, R>>>>,
}

impl <T: Request, R: Response> Application<T, R> {
    pub fn new(config: ServerConfig, router: Router<T,R>, middleware: Option<Vec<Box<dyn RequestMiddleware<T, R>>>>) -> Self {
        Self {
            config: config,
            router: router,
            middleware: middleware,
        }
    }
}


impl <T: Request, R: Response> Application<T, R> {
    fn get_bind(&self) -> String {
        format!("{:}:{:?}", self.config.address, self.config.port)
    }

    fn handle(&self, buffer: String) -> Result<RequestContext<T, R>, ServerError> {
        
        let mut ctx = RequestContext::<T, R>::new();

        // Parse request
        match parse::parse_into_request(buffer) {
            Ok(val) => {
                ctx.set_request(val);
            },
            Err(e) => return Err(e)
        }

        // Execute middleware pre request
        match &self.middleware {
            Some(val) => {
                for middleware in val.iter() {
                    match middleware.on_request(ctx) {
                        Ok(val) => {
                            ctx = val;
                        },
                        Err(e) => return Err(e),
                    }
                }
            },
            None => {},
        }

        // Dispatch route handler
        ctx = match self.router.dispatch(ctx) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        // Execute middleware pre response
        // This means errors wont go through this middleware...
        match &self.middleware {
            Some(val) => {
                for middleware in val.iter() {
                    match middleware.on_request(ctx) {
                        Ok(val) => {
                            ctx = val;
                        },
                        Err(e) => return Err(e),
                    }
                }
            },
            None => {},
        }

        Ok(ctx)

    }

    fn send_response(&self, stream: &mut TcpStream, response: String) {
        match stream.write(response.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error sending the response: {:?}", e);
            }
        }
    }

    fn close_connection(&self, stream: &mut TcpStream) {
        match stream.shutdown(Shutdown::Both) {
            Ok(_) => {}
            Err(e) => {
                println!("error closing the connection: {}", e);
            }
        }
    }

    pub fn serve(&self) {
        let listener = match TcpListener::bind(self.get_bind()) {
            Ok(val) => val,
            Err(e) => {
                println!("Error starting server: {:?}", e);
                panic!("Failed to start server on {:?}", self.get_bind());
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut _stream) => {
                    println!("accepted new connection");

                    // Process incoming data
                    let mut buf = String::new();
                    let mut stream_reader = BufReader::new(&_stream);

                    match stream_reader.read_line(&mut buf) {
                        Ok(_) => {},
                        Err(_) => {
                            // Send bad request?
                            // Assume this never happens for now
                        }
                    }

                    let response = match self.handle(buf) {
                        Ok(val) => parse::serialize_into_response(val.get_response()),
                        Err(e) => parse::serialize_error_into_response(e),
                    };


                    self.send_response(&mut _stream, response);
                    self.close_connection(&mut _stream);

                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}
