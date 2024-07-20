use crate::abnf::{CRLF, SP};
use crate::types;

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
// token = 1*<any CHAR except CTLs or tspecials>
const TOKEN: &str = r"[\x30-\x39\x41-\x5A\x61-\x7A]";
// tspecials = "(" | ")" | "<" | ">" | "@" | "," | ";" | ":" | "\" | <"> | "/" | "[" | "]" | "?" | "=" | "{" | "}" | SP  |  HT
const TSPECIALS: &str =
    r"\x28\x29\x3C\x3E\x40\x2C\x3B\x3A\x5C\x22\x2F\x5B\x5D\x3F\x3D\x7B\x7D\x20\x09";
// HTTP-Version   = "HTTP" "/" 1*DIGIT "." 1*DIGIT
const HTTP_VERSION: &str = r"HTTP\/[0-9\.]{1,3}";

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

#[cfg(test)]
mod tests {
    use super::*;

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
