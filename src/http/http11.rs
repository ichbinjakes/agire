use crate::http::abnf::{CRLF, SP, VCHAR};
use crate::http::types;

use regex;

// Hypertext Transfer Protocol -- HTTP/1.1 -- https://datatracker.ietf.org/doc/html/rfc2068

// REQUEST MESSAGE FORMAT:
//      In a http request each section is separated by a CRLF (\r\n)
//      Request line:    Get /index.html HTTP/1.1\r\n
//      Headers:         Host: localhost:4221\r\n
//      Entity:          (empty)\r\n

// Text = <any OCTET except CTLs, but including LWS>
// const TEXT: &str = r"[]";
// HEX = "A" | "B" | "C" | "D" | "E" | "F" | "a" | "b" | "c" | "d" | "e" | "f" | DIGIT
const HEX: &str = r"[[:xdigit:]]";

// token          = 1*tchar
// tchar          = "!" / "#" / "$" / "%" / "&" / "'" / "*"
//                 / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
//                 / DIGIT / ALPHA
// const TOKEN: &str = r"[\x30-\x39\x41-\x5A\x61-\x7A]";
const TOKEN: &str = r"[!#$%&'*+-.^_`|~[[:alnum:]]]";

// tspecials = "(" | ")" | "<" | ">" | "@" | "," | ";" | ":" | "\" | <"> | "/" | "[" | "]" | "?" | "=" | "{" | "}" | SP  |  HT

const TSPECIALS: &str =
    r"\x28\x29\x3C\x3E\x40\x2C\x3B\x3A\x5C\x22\x2F\x5B\x5D\x3F\x3D\x7B\x7D\x20\x09";

    // HTTP-Version   = "HTTP" "/" 1*DIGIT "." 1*DIGIT"GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n") {
        //     Some(val) => {
        //         println!("{:?}", val);
        //     },
        //     None => {},
        // }
const HTTP_VERSION: &str = r"HTTP\/[0-9\.]{1,3}";

// Parse header values from the request
pub fn parse_headers(request: &str) -> Vec<(String, String)> {
    // HTTP-message   = start-line CRLF
    //                *( field-line CRLF )
    //                CRLF
    //                [ message-body ]

    // field-line   = field-name ":" OWS field-value OWS

    // field-name     = token
    // field-value    = *field-content
    // field-content  = field-vchar
    //                  [ 1*( SP / HTAB / field-vchar ) field-vchar ]
    // field-vchar    = VCHAR / obs-text
    // obs-text       = %x80-FF
    // let re = regex::Regex::new(&format!(r"(?P<name>({TOKEN}+):{SP}+(?P<value>({VCHAR}|[\x80-\xFF])+)\r\n)")).unwrap();
    let re = regex::Regex::new(&format!(r"(?P<name>{TOKEN}+):{SP}+(?P<value>({VCHAR}|[\x80-\xFF])+)\r\n")).unwrap();

    let mut headers = Vec::<(String, String)>::new();

    for i in re.find_iter(request) {
        match re.captures(i.as_str()) {
            Some(val) => {
                let name = match val.name("name") {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let value = match val.name("value") {
                    Some(m) => m.as_str(),
                    None => "",
                };
                if name != "" && value != "" {
                    headers.push((String::from(name), String::from(value)));
                }
            },
            None => {},
        }
    }
    println!("headers: {:?} {}", &headers, request);
    headers
}

// Scan incoming request for the request line and extract details
pub fn parse_request_line(request: &str) -> Option<(String, String, String)> {
    // Request-Line = Method SP Request-URI SP HTTP-Version CRLF
    let request_line_rex = regex::Regex::new(&format!(
        r"(?P<method>{TOKEN}+){SP}(?P<uri>.*){SP}(?P<version>{HTTP_VERSION}){CRLF}"
    ))
    .unwrap();

    match request_line_rex.captures(request) {
        Some(c) => Some((
            String::from(&c["method"]),
            String::from(&c["uri"]),
            String::from(&c["version"]),
        )),
        None => None,
    }
}

pub fn parse_body(request: &str) -> Option<String> {
    let body_regex = regex::Regex::new(&format!(r"({CRLF}{CRLF})(?P<body>.*$)")).unwrap();
    match body_regex.captures(request) {
        Some(val) => {
            match val.name("body") {
                Some(body) => return Some(String::from(body.as_str())),
                None => return None,
            }
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_body() {
        let result = parse_body("GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n12345");
        assert_eq!(
            result, Some(String::from("12345"))
        );
    }

    #[test]
    fn test_parse_headers() {

        // let re = regex::Regex::new(&format!(r"(?P<name>{TOKEN}+):{SP}+(?P<value>({VCHAR}|[\x80-\xFF])+)\r\n")).unwrap();

        // match re.captures("User-Agent: curl/7.64.1\r\n") {
        //     Some(val) => {
        //         println!("{} {}", &val["name"], &val["value"]);
        //     },
        //     None => {},
        // }
        
        // for i in re.find_iter("GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n") {
        //     println!("{:?}", i);
        //     match re.captures(i.as_str()) {
        //         Some(val) => {
        //             let name = match val.name("name") {
        //                 Some(m) => m.as_str(),
        //                 None => "",
        //             };
        //             let value = match val.name("value") {
        //                 Some(m) => m.as_str(),
        //                 None => "",
        //             };
        //             if name != "" && value != "" {
        //                 println!("{}: {}", name, value);
        //             }
        //         },
        //         None => {},
        //     }
        // }

        // match re.find("GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n") {
        //     Some(val) => {
        //         println!("{:?}", val);
        //     },
        //     None => {},
        // }

        let result = parse_headers("GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        println!("{:?}", result);
        let desired = vec![
            (String::from("Host"), String::from("localhost:4221")),
            (String::from("User-Agent"), String::from("curl/7.64.1")),
            (String::from("Accept"), String::from("*/*")),
        ];
        assert_eq!(result, desired);
    }    

    #[test]
    fn test_parse_request_line() {
        let rl = "Get /index.html HTTP/1.1\r\n";
        let result = parse_request_line(rl);
        assert_eq!(
            result.unwrap(),
            (
                String::from("Get"),
                String::from("/index.html"),
                String::from("HTTP/1.1")
            )
        );

        let rl = "Post /api/v1/task HTTP/2.4\r\n";
        let result = parse_request_line(rl);
        assert_eq!(
            result.unwrap(),
            (
                String::from("Post"),
                String::from("/api/v1/task"),
                String::from("HTTP/2.4")
            )
        );
    }
}
