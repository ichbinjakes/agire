// use http::types::HttpMethod;
// use server::{routing,context};

mod http;
mod server;

use http::types::HttpMethod;
use server::application;
use server::context::{HttpRequest, HttpResponse, RequestContext};
use server::error::ServerError;
use server::routing;
use server::traits::{Request, Response};

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

use env_logger;
use log::{debug, error, info, trace, warn};

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

    let router = routing::Router::<HttpRequest, HttpResponse>::new(vec![
        routing::Route::new(
            String::from("/"),
            Box::new(root_route),
            vec![HttpMethod::Get],
        ),
        routing::Route::new(
            String::from("/echo/{str}"),
            Box::new(echo_route),
            vec![HttpMethod::Get],
        ),
    ]);

    let application = application::Application::new(cfg, router, None);

    application.serve();
}

fn root_route(
    ctx: RequestContext<HttpRequest, HttpResponse>,
) -> Result<RequestContext<HttpRequest, HttpResponse>, ServerError> {
    let mut ctx = ctx;
    let mut response = HttpResponse::new();
    response.set_status_code(200);
    ctx.set_response(response);
    Ok(ctx)
}

fn echo_route(
    ctx: RequestContext<HttpRequest, HttpResponse>,
) -> Result<RequestContext<HttpRequest, HttpResponse>, ServerError> {
    let mut ctx = ctx;
    let request = ctx.get_request();

    match request.get_path_param("str") {
        Some(val) => {
            let mut response = HttpResponse::new();
            response.set_status_code(200);
            response.set_body(val);
            ctx.set_response(response);
        }
        None => {
            return Err(ServerError::new(
                400,
                String::from("Missing parameter {str}"),
            ))
        }
    };

    Ok(ctx)
}
