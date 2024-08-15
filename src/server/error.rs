use crate::server::traits::Error;

pub struct ServerError {
    status_code: usize,
    detail: String,
}

impl Error for ServerError {
    fn get_status_code(&self) -> usize {
        self.status_code
    }
    fn get_detail(&self) -> String {
        self.detail.clone()
    }
}

pub enum StdServerError {
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    // NotAcceptable,
    // ProxyAuthenticationRequired,
    // RequestTimeout,
    // Conflict,
    // Gone,
    // ...
    UnprocessableContent,
    // ...
    InternalServerError,
    NotImplemented,
    // BadGateway,
    // ServiceUnavailable,
    // GatewayTimeout,
    HttpVersionNotSupported,
    // ...
}

impl StdServerError {
    pub fn to_error(&self) -> ServerError {
        match self {
            // 400s
            StdServerError::BadRequest => ServerError{status_code: 400, detail: String::from("Bad Request")},
            StdServerError::Unauthorized => ServerError{status_code:401, detail: String::from("Unauthorized")},
            StdServerError::PaymentRequired => ServerError{status_code:402, detail: String::from("Payment Required")},
            StdServerError::Forbidden => ServerError{status_code:403, detail: String::from("Forbidden")},
            StdServerError::NotFound => ServerError{status_code:404, detail: String::from("Not Found")},
            StdServerError::MethodNotAllowed => ServerError{status_code:405, detail: String::from("Method Not Allowed")},
            // StdServerError::NotAcceptable => ServerError{status_code:406, detail: String::from("Not Acceptable"),
            // StdServerError::ProxyAuthenticationRequired => ServerError{status_code:407, detail: String::from("Proxy Authentication Required"),
            // StdServerError::RequestTimeout => ServerError{status_code:408, detail: String::from("Request Timeout"),
            // StdServerError::Conflict => ServerError{status_code:409, detail: String::from("Conflict"),
            // StdServerError::Gone => ServerError{status_code: 410, detail: String::from("Gone"),
            // StdServerError::...
            StdServerError::UnprocessableContent => ServerError{status_code: 422, detail: String::from("Unprocessable Content")},
            // StdServerError::...
            // 500s
            StdServerError::InternalServerError =>ServerError{status_code: 500, detail: String::from("Unprocessable Content")},
            StdServerError::NotImplemented =>ServerError{status_code: 501, detail: String::from("Internal Server Error")},
            // StdServerError::BadGateway => ServerError{status_code: 502, detail: String::from("Bad Gateway")},
            // StdServerError::ServiceUnavailable => ServerError{status_code: 503, detail: String::from("Service Unavailable")},
            // StdServerError::GatewayTimeout => ServerError{status_code: 504, detail: String::from("Gateway Timeout")},
            StdServerError::HttpVersionNotSupported => ServerError{status_code: 505, detail: String::from("Http Version Not Supported")},
            // StdServerError::...
        }
    }
}