use crate::http::types::HttpMethod;

use super::context::RequestContext;
use super::error::ServerError;
use std::collections::HashMap;

/// Trait that defines methods a request type must have
/// Request types must implement this for use within route handlers
pub trait Request: Send + Sync {
    fn clone(&self) -> Self;

    /// Assign memory for the instance
    fn new() -> Self;

    /// Get the method type of the request
    fn get_method(&self) -> HttpMethod;

    /// Set the requests method
    fn set_method(&mut self, method: HttpMethod);

    /// Get a path parameter by name
    fn get_path(&self) -> String;

    /// Set the requests path
    fn set_path(&mut self, path: String);

    /// Get a path parameter by name
    fn get_path_param(&self, name: &str) -> Option<String>;

    // Set a path parameter by name
    fn set_path_param(&mut self, name: &str, value: &str);

    /// Get a query parameter by name
    fn get_query_param(&self, name: &str) -> Option<String>;

    /// Get a request header by name
    fn get_header(&self, name: &str) -> Option<String>;

    /// Set a header on the request
    fn set_header(&mut self, name: &str, value: &str);

    /// Get the requests body
    fn get_body(&self) -> String;

    /// Set the value of the body
    fn set_body(&mut self, body: String);
}

/// Trait that defines Response behaviour
pub trait Response: Send + Sync {
    fn clone(&self) -> Self;

    /// Assign memory for the response instance
    fn new() -> Self;

    /// Get the response status code
    fn get_status_code(&self) -> Option<usize>;

    /// Set the response status code
    fn set_status_code(&mut self, code: usize);

    /// Get a response header
    fn get_header(&self, name: &str) -> Option<String>;

    /// Get all headers
    fn get_headers(&self) -> &HashMap<String, String>;

    /// Set a response header
    fn set_header(&mut self, name: &str, value: &str);

    /// Get the response body
    fn get_body(&self) -> String;

    /// Set the response body
    fn set_body(&mut self, body: String);
}

/// Trait for middleware that operates on bytes from and to the client
// pub trait RawMiddleware {

//     ///
//     fn on_request(request: [u8]) -> [u8] {
//         return request;
//     }

//     ///
//     fn on_response(response: [u8]) -> [u8] {
//         return response;
//     }
// }

/// Trait for middleware that operates on impl Request & impl Response types
pub trait RequestMiddleware<T: Request, R: Response>: Send + Sync {
    // This function takes ownership of Request to mutate as needed
    fn on_request(&self, ctx: RequestContext<T, R>) -> Result<RequestContext<T, R>, ServerError> {
        Ok(ctx)
    }

    // This function takes ownership of Response to mutate as needed
    // A reference to the request is passed to extract information,
    // at this point I cannot think of a reason why request mould need mutation
    fn on_response(&self, ctx: RequestContext<T, R>) -> Result<RequestContext<T, R>, ServerError> {
        Ok(ctx)
    }
}

pub trait Error {
    fn get_status_code(&self) -> usize;
    fn get_detail(&self) -> String;
}
