use crate::http::types::HttpMethod;
use crate::http::uri;
use crate::server::context::RequestContext;
use crate::server::error::{ServerError, StdServerError};
use crate::server::traits::{Request, Response};

use log;
use regex;

pub struct Route<T: Request, R: Response> {
    pub path: String,
    pub func: Box<dyn Fn(RequestContext<T, R>) -> Result<RequestContext<T, R>, ServerError>>,
    pub methods: Vec<HttpMethod>,
    regex_path: Option<regex::Regex>,
}

impl<T: Request, R: Response> Route<T, R> {
    pub fn new(
        path: String,
        func: Box<dyn Fn(RequestContext<T, R>) -> Result<RequestContext<T, R>, ServerError>>,
        methods: Vec<HttpMethod>,
    ) -> Self {
        match convert_path_to_regex(&path) {
            Some(val) => {
                log::debug!(
                    "Route for path: {} generated a regex match pattern: {}.",
                    &path,
                    &val
                );
                let re = match regex::Regex::new(&val) {
                    Ok(val) => val,
                    Err(_) => panic!(
                        "Failed to convert the route path for regex matching: {}",
                        &path
                    ),
                };
                return Self {
                    path: path,
                    func: func,
                    methods: methods,
                    regex_path: Some(re),
                };
            }
            None => {
                log::debug!(
                    "Route for path: {} did not generate a regex match pattern.",
                    &path
                );
                return Self {
                    path: path,
                    func: func,
                    methods: methods,
                    regex_path: None,
                };
            }
        }
    }

    pub fn get_path_regex(&self) -> &Option<regex::Regex> {
        return &self.regex_path;
    }
}

pub struct Router<T: Request, R: Response> {
    routes: Vec<Route<T, R>>,
}

impl<T: Request, R: Response> Router<T, R> {
    pub fn new(routes: Vec<Route<T, R>>) -> Self {
        Self { routes: routes }
    }
}

impl<T: Request, R: Response> Router<T, R> {
    // Find the route requested via path matching
    fn match_path_to_route(&self, request: &impl Request) -> Result<&Route<T, R>, ServerError> {
        for route in self.routes.iter() {
            for method in route.methods.iter() {
                if *method == request.get_method() {
                    match route.get_path_regex() {
                        Some(val) => match val.find(&request.get_path()) {
                            Some(_) => return Ok(&route),
                            None => {}
                        },
                        None => {
                            if route.path == request.get_path() {
                                return Ok(&route);
                            }
                        },
                    }
                }
            }
        }
        Err(StdServerError::NotFound.to_error())
    }

    // Execute route handler
    pub fn dispatch(&self, ctx: RequestContext<T, R>) -> Result<RequestContext<T, R>, ServerError> {
        let mut ctx = ctx;
        let request = ctx.get_request();

        let route = match self.match_path_to_route(request) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        match route.get_path_regex() {
            Some(re) => {
                match extract_path_params(&request.get_path(), re) {
                    Ok(val) => {
                        // There should be a better way to do this -> pass request and response around
                        let mut request = ctx.get_request().clone();
                        for (name, value) in val.iter() {
                            // log::debug!("Setting path parameter: {} {}", name, value);
                            request.set_path_param(name, value);
                            // log::debug!("Getting path parameter: str {}", request.get_path_param("str").unwrap())
                        }
                        ctx.set_request(request);
                    }
                    Err(e) => return Err(e),
                }
            }
            None => {}
        }
        let f = &route.func;
        f(ctx)
    }
}

fn extract_path_params(
    path: &str,
    path_regex: &regex::Regex,
) -> Result<Vec<(String, String)>, ServerError> {
    let mut path_params = Vec::<(String, String)>::new();

    let caps = match path_regex.captures(path) {
        Some(val) => val,
        None => return Err(StdServerError::BadRequest.to_error()),
    };

    for name in path_regex.capture_names() {
        match name {
            Some(val) => path_params.push((String::from(val), String::from(&caps[val]))),
            None => {
                // log::error!("Failed to find a path parameter from the request: {:?}", path_regex.capture_names());
                // return Err(StdServerError::BadRequest.to_error());
            }
        }
    }

    Ok(path_params)
}

/// Function
fn convert_path_to_regex(path: &str) -> Option<String> {
    // regex to extract params from request path - p-char: unreserved / pct-encoded / sub-delims / ":" / "@"
    let path_re = format!(
        r"[{}{}{}\:\@]+",
        uri::UNRESERVED,
        uri::PCT_ENCODED,
        uri::SUB_DELIMS
    );
    // regex to find param in compiled path
    let param_re = regex::Regex::new(r"\{[[:alpha:]]+\}").unwrap();

    match param_re.find(path) {
        Some(_) => {}
        None => return None,
    }

    // 1. find the strings surrounded by {}, only alpha num can be used
    let path_components = path.split('/');
    let mut new_components = vec![String::from("^")];

    for component in path_components.filter(|i| *i != "") {
        // println!("Comp: {component}");
        new_components.push(String::from("/"));
        match param_re.find(component) {
            Some(_) => {
                let param_name = String::from(component.trim_matches(&['{', '}']));
                new_components.push(format!("(?P<{}>{})", param_name, path_re));
            }
            None => new_components.push(String::from(component)),
        }
    }

    new_components.push(String::from("$"));

    Some(String::from_iter(new_components))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_convert_path_to_regex() {
//     //     // /echo/{str} -> /echo/(?P<str>PATH_RE)
//     //     assert_eq!(
//     //         convert_path_to_regex("/echo/{str}"),
//     //         "^/echo/(?P<str>[[::alpha::]]+)$"
//     //     );
//     //     // /echo/{num}/{str} -> /echo/(?P<num>PATH_RE)/(?P<str>PATH_RE)
//     //     assert_eq!(
//     //         convert_path_to_regex("/echo/{num}/{str}"),
//     //         "^/echo/(?P<num>[[::alpha::]]+)/(?P<str>[[::alpha::]]+)$"
//     //     );
//     // }

//     #[test]
//     fn test_path_to_params() {
//         // single param
//         let path = String::from("/echo/{str}");
//         let repath = convert_path_to_regex(&path);
//         let re = regex::Regex::new(&repath).unwrap();
//         // let re = regex::Regex::new(&convert_path_to_regex(&path)).unwrap();

//         let mat = re.find("/echo/hello").unwrap();
//         assert_eq!(mat.is_empty(), false);

//         match re.captures("/echo/hello") {
//             Some(val) => assert_eq!(&val["str"], "hello"),
//             None => assert!(false),
//         }

//         // double param
//         let path = String::from("/echo/{num}/{str}");
//         let repath = convert_path_to_regex(&path);
//         let re = regex::Regex::new(&repath).unwrap();
//         match re.captures("/echo/two/hello") {
//             Some(val) => {
//                 assert_eq!(&val["num"], "two");
//                 assert_eq!(&val["str"], "hello");
//             }
//             None => assert!(false),
//         }
//     }

//     #[test]
//     fn test_regex_for_path_match() {
//         let root_re = String::from(r"^/$");
//         let single_path_re = String::from(r"^/([[:alpha:]]+){1}$");
//         let multiple_path_re = String::from(r"^/([[:alpha:]]+/)+([[:alpha:]]+)${1}");
//         let test_re = format!(r"({})|({})|({})", root_re, single_path_re, multiple_path_re);

//         // /
//         let re = regex::Regex::new(&root_re).unwrap();
//         let mat = re.find("/").unwrap();
//         assert_eq!(mat.is_empty(), false);

//         // /echo
//         let re = regex::Regex::new(&single_path_re).unwrap();
//         let mat = re.find("/echo").unwrap();
//         // println!("{}", mat.as_str());
//         assert_eq!(mat.is_empty(), false);

//         // /echo/str/...
//         let re = regex::Regex::new(&multiple_path_re).unwrap();
//         let mat = re.find("/echo/str").unwrap();
//         // println!("{}", mat.as_str());
//         assert_eq!(mat.is_empty(), false);

//         // all
//         let re = regex::Regex::new(&test_re).unwrap();

//         let mat = re.find("/").unwrap();
//         assert_eq!(mat.is_empty(), false);

//         let mat = re.find("/echo").unwrap();
//         assert_eq!(mat.is_empty(), false);

//         let mat = re.find("/echo/str").unwrap();
//         assert_eq!(mat.is_empty(), false);

//         match re.find("/echo/") {
//             Some(val) => {
//                 assert_eq!(val.is_empty(), true);
//             }
//             None => {}
//         }
//     }
// }
