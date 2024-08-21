use crate::http::http11;
use crate::http::types::HttpMethod;
use crate::server::error::{ServerError, StdServerError};
use crate::server::traits::{Error, Request, Response};

use log::{debug, error, info, trace, warn};

/// Parse the incoming request bytes into a struct that implements Request
pub fn parse_into_request<T: Request>(raw: String) -> Result<T, ServerError> {
    let mut request = T::new();

    // Parse Request Line
    let rl = match http11::parse_request_line(&raw) {
        Some(val) => val,
        None => {
            debug!("Failed to parse request line.");
            debug!("{}", &raw);
            return Err(StdServerError::BadRequest.to_error());
        }
    };

    // Request Method
    match HttpMethod::from_str(&rl.0) {
        Ok(val) => request.set_method(val),
        Err(_) => {
            debug!("Failed to parse request method.");
            debug!("{}", &raw);
            return Err(StdServerError::BadRequest.to_error());
        }
    }

    // Path
    let uri = rl.1;
    // Path + Query params not currently supported
    request.set_path(uri);

    // Headers
    for (name, value) in http11::parse_headers(&raw) {
        request.set_header(&name, &value);
    }

    // Body
    // Not currently supported

    Ok(request)
}

/// Serialise a struct the implments Response into raw bytes for transfer to client
pub fn serialize_into_response<R: Response>(response: &R) -> String {
    let body = response.get_body();
    
    let status_code = match response.get_status_code() {
        Some(val) => val,
        None => {
            return serialize_error_into_response(StdServerError::InternalServerError.to_error());
        }
    };
    
    let mut header = String::new();
    match response.get_header("Content-Length") {
        Some(_) => {},
        None => {
            header.push_str(&format!("{}: {}\r\n", "Content-Length", body.len().to_string().as_str()));
        }
    }
    for (key, val) in response.get_headers().iter() {
        header.push_str(&format!("{}: {}\r\n", key, val));
    }

    let status_line = format!("HTTP/1.1 {} OK", status_code);
    let response = format!("{}\r\n{}\r\n{}", status_line, header, body);

    debug!("{}", response);

    response
}

pub fn serialize_error_into_response(error: impl Error) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n\r\n",
        error.get_status_code(),
        error.get_detail()
    )
}
