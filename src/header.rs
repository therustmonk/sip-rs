use std::fmt::{self, Display};
use std::str::FromStr;

use uuid::Uuid;

// HEADERS

pub trait Header: Display {
}

pub struct Contact {
    //pub hops: u16,
}

impl Header for Contact {
}

impl Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contact: *")
    }
}

pub struct MaxForwards {
    pub hops: u16,
}

impl Header for MaxForwards {
}

impl Display for MaxForwards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Max-Forwards: {}", self.hops)
    }
}

pub struct Via {
    pub version: SipVersion,
    pub transport: String,
    pub host: SipHost,
    pub branch: Option<String>,
    pub rport: Option<u16>,
}

impl Header for Via {
}

impl Display for Via {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Via: {}/{} {}", self.version, self.transport, self.host));
        if let Some(ref branch) = self.branch {
            try!(write!(f, ";branch={}", branch));
        }
        if let Some(ref rport) = self.rport {
            try!(write!(f, ";rport={}", rport))
        }
        Ok(())
    }
}

pub struct CallId {
    pub id: String,
}

impl Header for CallId {
}

impl Display for CallId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Call-ID: {}", self.id)
    }
}

impl CallId {
    pub fn random_with_host(host: &SipHost) -> Self {
        let uid = Uuid::new_v4();
        CallId { id: format!("{}@{}", uid, host.address) }
    }

    pub fn random() -> Self {
        let uid = Uuid::new_v4();
        CallId { id: uid.to_string() }
    }
}

pub struct ContentLength {
    pub bytes: u16,
}

impl Header for ContentLength {
}

impl Display for ContentLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content-Length: {}", self.bytes)
    }
}

pub struct CSeq {
    pub number: u32,
    pub method: Method,
}

impl Header for CSeq {
}

impl Display for CSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSeq: {} {}", self.number, self.method)
    }
}

pub struct UserAgent {
    pub name: String,
}

impl Header for UserAgent {
}

impl Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User-Agent: {}", self.name)
    }
}

pub struct From {
    pub uri: SipUri,
    pub tag: Option<String>,
}

impl Header for From {
}

impl Display for From {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "From: <{}>", self.uri));
        if let Some(ref tag) = self.tag {
            try!(write!(f, ";tag={}", tag));
        }
        Ok(())
    }
}

pub struct To {
    pub uri: SipUri,
    pub tag: Option<String>,
}

impl Header for To {
}

impl Display for To {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "To: <{}>", self.uri));
        if let Some(ref tag) = self.tag {
            try!(write!(f, ";tag={}", tag));
        }
        Ok(())
    }
}


// MESSAGES

pub enum ParseError {
    InvalidMethod,
    InvalidSipVersion,
}

//pub type ParseResult<T> = Result<T, ParseError>

pub enum Method {
    Register,
    Invite,
    Ack,
    Cancel,
    Buy,
    Options,
}

// Constats to prevent typo mistakes
const REGISTER: &'static str = "REGISTER";
const INVITE: &'static str = "INVITE";
const ACK: &'static str = "ACK";
const CANCEL: &'static str = "CANCEL";
const BUY: &'static str = "BUY";
const OPTIONS: &'static str = "OPTIONS";

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Method::*;
        f.write_str(match *self {
            Register => REGISTER,
            Invite => INVITE,
            Ack => ACK,
            Cancel => CANCEL,
            Buy => BUY,
            Options => OPTIONS,
        })
    }
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Method::*;
        match s {
            REGISTER => Ok(Register),
            INVITE => Ok(Invite),
            ACK => Ok(Ack),
            CANCEL => Ok(Cancel),
            BUY => Ok(Buy),
            OPTIONS => Ok(Options),
            _ => Err(ParseError::InvalidMethod),
        }
    }
}

#[derive(Clone)]
pub struct SipHost {
    pub address: String,
    pub port: Option<u16>,
}

impl Display for SipHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SipHost { ref address, port: Some( ref port) } =>
                write!(f, "{}:{}", address, port),
            SipHost { ref address, port: None } =>
                write!(f, "{}", address),
        }
    }
}

#[derive(Clone)]
pub struct SipUri {
    pub user: Option<String>,
    pub host: SipHost,
}

impl Display for SipUri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SipUri { user: Some(ref user), ref host } =>
                write!(f, "sip:{}@{}", user, host),
            SipUri { user: None, ref host } =>
                write!(f, "sip:{}", host),
        }
    }
}

impl SipUri {
    pub fn from_host(host: &str) -> Self {
        SipUri {
            user: None,
            host: SipHost {
                address: host.to_string(),
                port: None
            }
        }
    }
}

pub enum SipVersion {
    Sip,
    Sip2,
}

const SIP: &'static str = "SIP";
const SIP2: &'static str = "SIP/2.0";

impl Display for SipVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SipVersion::*;
        f.write_str(match *self {
            Sip => SIP,
            Sip2 => SIP2,
        })
    }
}

impl FromStr for SipVersion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::SipVersion::*;
        match s {
            SIP => Ok(Sip),
            SIP2 => Ok(Sip2),
            _ => Err(ParseError::InvalidSipVersion),
        }
    }
}

pub struct Request {
    pub method: Method,
    pub uri: SipUri,
    pub version: SipVersion,
    pub headers: Vec<Box<Header>>,
}

impl Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {} {}", self.method, self.uri, self.version));
        for header in &self.headers {
            try!(write!(f, "\n{}", header));
        }
        write!(f, "\n\n")
    }
}

/*
pub struct RequestLine {
    pub method: Method,
    pub uri: SipUri,
    pub version: SipVersion,
}

impl Display for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.method, self.uri, self.version)
    }
}

pub enum StatusCode {
}

pub struct StatusLine {
    pub version: SipVersion,
    pub code: StatusCode,
}

impl Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StatusLine")
    }
}

pub enum Header {
    Accept,
    AcceptEncoding,
    AcceptLanguage,
}

pub enum StartLine {
    Request(RequestLine),
    Status(StatusLine),
}

impl Display for StartLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::StartLine::*;
        match *self {
            Request(ref x) => Display::fmt(x, f),
            Status(ref x) => Display::fmt(x, f),
        }
    }
}

pub struct SipMessage {
    pub start: StartLine,
    pub headers: Vec<Header>,
}

impl Display for SipMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.start)
    }
}


fn parse(buf: &[u8]) -> SipMessage {
    unimplemented!();
}

*/

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let data = "REGISTER <> SIP/2.0".as_bytes();
    }

    #[test]
    fn stringify_test() {
        let message = SipMessage {
            start: StartLine::Request(RequestLine {
                method: Method::Register,
                uri: SipUri {
                    user: "user".to_string(),
                    host: "localhost".to_string(),
                },
                version: SipVersion::Sip2,
            }),
            headers: Vec::new(),
        };

        let message = message.to_string();
        let mut iter = message.lines();
        assert_eq!("REGISTER sip:user@localhost SIP/2.0", iter.next().unwrap());
    }

}
*/
