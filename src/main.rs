
// use http::types::HttpMethod;
// use server::{routing,context};

mod http;
mod server;

use http::types::HttpMethod;
use server::context::{HttpRequest,HttpResponse,RequestContext};
use server::routing;
use server::application;
use server::traits::Response;
use server::error::ServerError;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

use env_logger;
use log::{info, warn, debug, trace, error};


fn main() {
    env_logger::init();

    info!("This is info");
    warn!("This is warn");
    debug!("This is debug");
    trace!("This is trace");
    error!("This is error");

    let cfg = application::ServerConfig {
        address: String::from("127.0.0.1"),
        port: 4221,
    };

    let router = routing::Router::<HttpRequest, HttpResponse>::new(
        vec![
            routing::Route {
                path: String::from("/"),
                func: Box::new(root_route),
                methods: vec![HttpMethod::Get],
            },
        ]
    );

    let application = application::Application::new(
        cfg, router, None
    );

    application.serve();

}

fn root_route(ctx: RequestContext<HttpRequest, HttpResponse>) -> Result<RequestContext<HttpRequest, HttpResponse>, ServerError> {
    let mut ctx = ctx;
    let mut response = HttpResponse::new();
    response.set_status_code(200);
    ctx.set_response(response);
    Ok(ctx)
}