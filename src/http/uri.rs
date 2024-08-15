use regex::Regex;

// URI rfc3986 //

// scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
pub const SCHEME: &str = r"[[:alpha:]]([[[:alnum:]]\+\-\.])*";

// authority = [ userinfo "@" ] host [ ":" port ]


// userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )


// host = IP-literal / IPv4address / reg-name


// IP-literal = "[" ( IPv6address / IPvFuture  ) "]"


// IPvFuture  = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )


// IPv6address =                            6( h16 ":" ) ls32
//             /                       "::" 5( h16 ":" ) ls32
//             / [               h16 ] "::" 4( h16 ":" ) ls32
//             / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
//             / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
//             / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
//             / [ *4( h16 ":" ) h16 ] "::"              ls32
//             / [ *5( h16 ":" ) h16 ] "::"              h16
//             / [ *6( h16 ":" ) h16 ] "::"

// ls32        = ( h16 ":" h16 ) / IPv4address
//              ; least-significant 32 bits of address

// h16         = 1*4HEXDIG
//              ; 16 bits of address represented in hexadecimal

// IPv4address = dec-octet "." dec-octet "." dec-octet "." dec-octet

// dec-octet   = DIGIT                 ; 0-9
//             / %x31-39 DIGIT         ; 10-99
//             / "1" 2DIGIT            ; 100-199
//             / "2" %x30-34 DIGIT     ; 200-249
//             / "25" %x30-35          ; 250-255

// reg-name    = *( unreserved / pct-encoded / sub-delims )

//  port        = *DIGIT

// path          = path-abempty    ; begins with "/" or is empty
//               / path-absolute   ; begins with "/" but not "//"
//               / path-noscheme   ; begins with a non-colon segment
//               / path-rootless   ; begins with a segment
//               / path-empty      ; zero characters

// path-abempty  = *( "/" segment )
// path-absolute = "/" [ segment-nz *( "/" segment ) ]
// path-noscheme = segment-nz-nc *( "/" segment )
// path-rootless = segment-nz *( "/" segment )
// path-empty    = 0<pchar>
// segment       = *pchar
// segment-nz    = 1*pchar
// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
//               ; non-zero-length segment without any colon ":"

// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"

// query       = *( pchar / "/" / "?" )

// fragment    = *( pchar / "/" / "?" )

// pct-encoded   = "%" HEXDIG HEXDIG
pub const PCT_ENCODED: &str = r"\%[[:xdigit:]][[:xdigit:]]";

// unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
pub const UNRESERVED: &str = r"[[:alnum:]]\-\.\_\~";
// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
pub const GEN_DELIMS: &str = r"\:\\\?\#\[\]\@";
// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
//               / "*" / "+" / "," / ";" / "="
pub const SUB_DELIMS: &str = r"\!\$\&\'\(\)\*\+\,\;\=";
// reserved      = gen-delims / sub-delims
pub static RESERVED: &str = &format!(r"[{}{}]", String::from(GEN_DELIMS), String::from(SUB_DELIMS));

// URI-reference = URI / relative-ref

// relative-ref  = relative-part [ "?" query ] [ "#" fragment ]

// relative-part = "//" authority path-abempty
//               / path-absolute
//               / path-noscheme
//               / path-empty


// absolute-URI  = scheme ":" hier-part [ "?" query ]