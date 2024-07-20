use regex::Regex;

// ABNF rules: https://datatracker.ietf.org/doc/html/rfc5234#autoid-24

// ALPHA =  %x41-5A / %x61-7A   ; A-Z / a-z
pub const ALPHA: &str = r"[[:alpha:]]";
// BIT  =  "0" / "1"
pub const BIT: &str = r"[01]";
// CHAR =  %x01-7F ; any 7-bit US-ASCII character, excluding NUL
pub const CHAR: &str = r"[\x01-\x7F]";
// CR =  %x0D ; carriage return
pub const CR: &str = r"\r";
// CRLF = CR LF ; internet standard newline
pub const CRLF: &str = r"\r\n";
// CTL = <any US-ASCII control character (octets 0 - 31) and DEL (127)>
pub const CTL: &str = r"[[:cntrl:]]";
// DIGIT =  %x30-39 ; 0-9
pub const DIGIT: &str = r"[[:digit:]]";
// DQUOTE = %x22 ; " (Double Quote)
pub const DQUOTE: &str = r"\x22";
// HEXDIG =  DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
pub const HEXDIGIT: &str = r"[[:xdigit:]]";
// HTAB =  %x09 ; horizontal tab
pub const HTAB: &str = r"\x09";
// LF =  %x0A ; linefeed
pub const LF: &str = r"\x0A";
// LWS = [CRLF] 1*( SP | HT ) -- OR -- LWSP = *(WSP / CRLF WSP)
pub const LWS: &str = "(\r\n)?[[:blank:]]+";
// OCTET = %x00-FF ; 8 bits of data
pub const OCTET: &str = r"[\x00-\xFF]";
// SP =  %x20
pub const SP: &str = r"\x20";
// VCHAR = %x21-7E ; visible (printing) characters
pub const VCHAR: &str = r"[\x21-\x7E]";
// WSP = SP / HTAB
pub const WSP: &str = r"[[:blank:]]";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha() {
        let re = Regex::new(ALPHA).unwrap();
        let mat = re.find("123a321").unwrap();
        assert_eq!(mat.is_empty(), false);
        assert_eq!(mat.start(), 3);
    }

    #[test]
    fn test_char() {
        let re = Regex::new(CHAR).unwrap();
        let mat = re.find(" ").unwrap();
        assert_eq!(mat.is_empty(), false);
        assert_eq!(mat.start(), 0);

        let mat = re.find("~").unwrap();
        assert_eq!(mat.is_empty(), false);
        assert_eq!(mat.start(), 0);
    }

    #[test]
    fn test_crlf() {
        let re = Regex::new(CRLF).unwrap();
        let mat = re.find("aaaa\r\n").unwrap();
        assert_eq!(mat.is_empty(), false);
        assert_eq!(mat.start(), 4);
    }

    #[test]
    fn test_dquote() {
        let re = Regex::new(DQUOTE).unwrap();
        let mat = re.find("dd\"dd").unwrap();
        assert_eq!(mat.is_empty(), false);
        assert_eq!(mat.start(), 2);
    }

    #[test]
    fn test_lwsp() {
        let re = Regex::new(LWS).unwrap();
        let mat = re.find("hello \r\n").unwrap();
        assert_eq!(mat.is_empty(), false);
        assert_eq!(mat.start(), 5);
        assert_eq!(mat.end(), 6);

        match re.find("hello\r\n") {
            Some(_) => assert!(false),
            None => assert!(true),
        }

        let mat = re.find("hello\r\n ").unwrap();
        assert_eq!(mat.is_empty(), false);
        assert_eq!(mat.start(), 5);
        assert_eq!(mat.end(), 8)
    }
}
