use crate::http::error;

#[derive(Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Unset,
    Get,
    Post,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Result<Self, error::HttpError> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err(error::HttpError {
                code: 400,
                detail: String::from("Unsupported HTTP Method"),
            }),
        }
    }
}

enum HttpVersion {
    V1_0,
    V1_1,
    V2,
    V3,
}

impl HttpVersion {
    fn from_str(s: &str) -> Result<Self, error::HttpError> {
        match s {
            "HTTP/1.0" => Ok(Self::V1_0),
            "HTTP/1.1" => Ok(Self::V1_1),
            "HTTP/2" => Ok(Self::V2),
            "HTTP/3" => Ok(Self::V3),
            _ => Err(error::HttpError {
                code: 400,
                detail: String::from("Unsupported HTTP Version"),
            }),
        }
    }
}

struct RequestLine {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
}
