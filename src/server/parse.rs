use crate::http::types::HttpMethod;
use crate::server::traits::{Request,Response,Error};
use crate::server::error::{StdServerError, ServerError};
use crate::http::http11;

use log::{info, warn, debug, trace, error};


/// Parse the incoming request bytes into a struct that implements Request
pub fn parse_into_request<T: Request>(raw: String) -> Result<T, ServerError> {
    let mut request = T::new();

    // let raw_request = match String::from_utf8(raw.to_vec()) {
    //     Ok(val) => val,
    //     Err(_) => return Err(StdServerError::BadRequest.to_error()),
    // };

    // Parse Request Line
    let rl = match http11::parse_request_line(&raw) {
        Some(val) => val,
        None => {
            debug!("Failed to parse request line.");
            debug!("{}", &raw);
            return Err(StdServerError::BadRequest.to_error());
        },
    };

    // Request Method
    match HttpMethod::from_str(&rl.0) {
        Ok(val) => request.set_method(val),
        Err(_) => {
            debug!("Failed to parse request method.");
            debug!("{}", &raw);
            return Err(StdServerError::BadRequest.to_error());
        },
    }

    // Path 
    let uri = rl.1;
    // Path + Query params not currently supported
    request.set_path(uri);

    // Headers
    // Not currently supported

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
        },
    };

    let status_line = format!("HTTP/1.1 {} Ok", status_code); 
    let response = format!("{}\r\n\r\n{}", status_line, body);
    
    debug!("{}", response);

    response
}

pub fn serialize_error_into_response(error: impl Error) -> String {
    format!("HTTP/1.1 {} {}\r\n\r\n", error.get_status_code(), error.get_detail())
}