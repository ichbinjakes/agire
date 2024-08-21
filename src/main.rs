// use http::types::HttpMethod;
// use server::{routing,context};

mod http;
mod server;

use http::types::HttpMethod;
use server::error::StdServerError;
use server::application;
use server::context::{HttpRequest, HttpResponse, RequestContext};
use server::error::ServerError;
use server::routing;
use server::traits::{Request, Response};

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path;
use std::fs;

use env_logger;
use log;

fn main() {
    env_logger::init();

    log::info!("This is info");
    log::warn!("This is warn");
    log::debug!("This is debug");
    log::trace!("This is trace");
    log::error!("This is error");

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
        routing::Route::new(
            String::from("/user-agent"),
            Box::new(user_agent_route),
            vec![HttpMethod::Get],
        ),
        routing::Route::new(
            String::from("/files/{filename}"),
            Box::new(files_get_route),
            vec![HttpMethod::Get],
        ),
        routing::Route::new(
            String::from("/files/{filename}"),
            Box::new(files_post_route),
            vec![HttpMethod::Post],
        ),
    ]);

    let app = application::Application::new(cfg, router);

    application::serve(app);
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
            response.set_header("Content-Type", "text/plain");
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

fn user_agent_route(
    ctx: RequestContext<HttpRequest, HttpResponse>,
) -> Result<RequestContext<HttpRequest, HttpResponse>, ServerError> {
    let mut ctx = ctx;
    let request = ctx.get_request();

    match request.get_header("User-Agent") {
        Some(val) => {
            let mut response = HttpResponse::new();
            response.set_status_code(200);
            response.set_body(val);
            response.set_header("Content-Type", "text/plain");
            ctx.set_response(response);
        },
        None => {
            return Err(ServerError::new(
                400,
                String::from("Missing User-Agent header"),
            ))
        }
    }
    Ok(ctx)
}

fn files_get_route(
    ctx: RequestContext<HttpRequest, HttpResponse>,
) -> Result<RequestContext<HttpRequest, HttpResponse>, ServerError> {
    let mut ctx = ctx;
    let request = ctx.get_request();

    let mut is_next = false;
    let mut directory = String::new();
    for arg in std::env::args() {
        if is_next {
            directory = arg;
            break;
        }
        if arg == "--directory" {
            is_next = true;
        }
    }
    log::debug!("Directory is set to: {}", directory);
    
    let filename = match request.get_path_param("filename") {
        Some(val) => val,
        None => {
            return Err(StdServerError::BadRequest.to_error());
        },
    };

    let file_path = path::Path::new(&directory);
    let file_path = file_path.join(filename);

    if !file_path.exists() {
        log::error!("File doesn't exist: {:?}", file_path.as_os_str());
        return Err(StdServerError::NotFound.to_error());
    }

    let mut file_contents = String::new();
    let mut fh = match std::fs::File::open(file_path) {
        Ok(val) => val,
        Err(e) => {
            log::error!("{:?}", e);
            return Err(StdServerError::InternalServerError.to_error());
        }
    };
    match fh.read_to_string(&mut file_contents) {
        Ok(_) => {},
        Err(e) => {
            log::error!("{:?}", e);
            return Err(StdServerError::InternalServerError.to_error());
        }
    }

    let mut response = HttpResponse::new();
    response.set_status_code(200);
    response.set_body(file_contents);
    response.set_header("Content-Type", "application/octet-stream");
    ctx.set_response(response);

    Ok(ctx)
}

fn files_post_route(
    ctx: RequestContext<HttpRequest, HttpResponse>,
) -> Result<RequestContext<HttpRequest, HttpResponse>, ServerError> {
    let mut ctx = ctx;
    let request = ctx.get_request();

    let mut is_next = false;
    let mut directory = String::new();
    for arg in std::env::args() {
        if is_next {
            directory = arg;
            break;
        }
        if arg == "--directory" {
            is_next = true;
        }
    }
    
    let filename = match request.get_path_param("filename") {
        Some(val) => val,
        None => {
            return Err(StdServerError::BadRequest.to_error());
        },
    };

    let file_path = path::Path::new(&directory);
    let file_path = file_path.join(filename);

    if file_path.exists() {
        log::error!("File already exist: {:?}", file_path.as_os_str());
        return Err(ServerError::new(
            403,
            String::from("File already exists"),
        ));
    }

    let file_contents = request.get_body();

    let mut fh = match std::fs::File::create(file_path) {
        Ok(val) => val,
        Err(e) => {
            log::error!("{:?}", e);
            return Err(StdServerError::InternalServerError.to_error());
        }
    };
    match fh.write_all(file_contents.as_bytes()) {
        Ok(_) => {},
        Err(e) => {
            log::error!("{:?}", e);
            return Err(StdServerError::InternalServerError.to_error());
        }
    }

    let mut response = HttpResponse::new();
    response.set_status_code(201);
    response.set_body(String::from("Created"));
    response.set_header("Content-Type", "application/octet-stream");
    ctx.set_response(response);

    Ok(ctx)
}
