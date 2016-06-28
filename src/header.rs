use std::fmt::{self, Display};
use std::str::FromStr;

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

pub struct RequestUri {
}

pub enum SipVersion {
    Sip,
    Sip2,
}

const SIP: &'static str = "SIP";
const SIP2: &'static str = "SIP2";

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
pub struct RequestLine {
    pub method: Method,
    pub uri: RequestUri,
    pub version: SipVersion,
}

impl Display for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <URI> {}", self.method, self.version)
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
    RequestLine(RequestLine),
    StatusLine(StatusLine),
}

impl Display for StartLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::StartLine::*;
        match *self {
            RequestLine(ref x) => Display::fmt(x, f),
            StatusLine(ref x) => Display::fmt(x, f),
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
