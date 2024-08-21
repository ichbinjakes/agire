use crate::http::types::HttpMethod;
use crate::server::traits::{Request, Response};

use std::collections::HashMap;

use log;

pub struct RequestContext<T: Request, R: Response> {
    request: T,
    response: R,
}

impl<T: Request, R: Response> RequestContext<T, R> {
    pub fn new() -> Self {
        Self {
            request: T::new(),
            response: R::new(),
        }
    }

    pub fn get_request(&self) -> &T {
        &self.request
    }

    pub fn set_request(&mut self, request: T) {
        self.request = request;
    }

    pub fn get_response(&self) -> &R {
        &self.response
    }

    pub fn set_response(&mut self, response: R) {
        self.response = response;
    }
}

pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    path_params: HashMap<String, String>,
    query_params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: String,
}

impl Request for HttpRequest {
    fn clone(&self) -> Self {
        Self {
            method: self.method.clone(),
            path: self.path.clone(),
            path_params: self.path_params.clone(),
            query_params: self.query_params.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }

    fn new() -> Self {
        Self {
            method: HttpMethod::Unset,
            path: String::new(),
            path_params: HashMap::new(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    fn get_method(&self) -> HttpMethod {
        self.method.clone()
    }

    fn set_method(&mut self, method: HttpMethod) {
        self.method = method;
    }

    fn get_path(&self) -> String {
        return self.path.clone();
    }

    fn set_path(&mut self, path: String) {
        self.path = path;
    }

    fn get_path_param(&self, name: &str) -> Option<String> {
        match self.path_params.get(name) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    fn set_path_param(&mut self, name: &str, value: &str) {
        self.path_params
            .insert(String::from(name), String::from(value));
    }

    fn get_query_param(&self, name: &str) -> Option<String> {
        match self.query_params.get(name) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    fn get_header(&self, name: &str) -> Option<String> {
        match self.headers.get(name) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    fn get_body(&self) -> String {
        self.body.clone()
    }

    fn set_header(&mut self, name: &str, value: &str) {
        log::debug!("Setting header: {}: {}", name, value);
        self.headers.insert(String::from(name), String::from(value));
    }

    fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

#[derive(Clone)]
pub struct HttpResponse {
    status_code: Option<usize>,
    headers: HashMap<String, String>,
    body: String,
}

impl Response for HttpResponse {
    fn clone(&self) -> Self {
        Self {
            status_code: self.status_code.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }
    fn new() -> Self {
        Self {
            status_code: None,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    fn get_status_code(&self) -> Option<usize> {
        self.status_code.clone()
    }

    fn set_status_code(&mut self, code: usize) {
        self.status_code = Some(code)
    }

    fn get_header(&self, name: &str) -> Option<String> {
        match self.headers.get(name) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(String::from(name), String::from(value));
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn get_body(&self) -> String {
        self.body.clone()
    }

    fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

impl HttpResponse {
    fn plaintext_response(body: String) -> HttpResponse {
        let mut response = HttpResponse::new();
        response.set_header("Content-Type", "text/plain");
        response.set_body(body);
        response
    }
}
