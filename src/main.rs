extern crate httparse;

pub mod header;

use std::net::UdpSocket;
use header::*;

fn main() {

    println!("Connecting to SIP server...");

    let me = SipUri {
        user: Some("100".to_string()),
        host: SipHost {
            address: "192.168.1.143".to_string(),
            port: None,
        },
    };

    let headers: Vec<Box<Header>> = vec![
        Box::new(MaxForwards { hops: 70 }),
        Box::new(ContentLength { bytes: 0 }),
        Box::new(CSeq { number: 12340, method: Method::Register }),
        Box::new(From { uri: me.clone() }),
        Box::new(To { uri: me.clone() }),
    ];

    let message = Request {
        method: Method::Register,
        uri: SipUri::from_host("192.168.1.143"),
        version: SipVersion::Sip2,
        headers: headers,
    };

    let message = message.to_string();

    let udp = UdpSocket::bind("0.0.0.0:5061").unwrap();
    udp.connect("192.168.1.143:5060").unwrap();
    udp.send(message.as_bytes()).unwrap();

    let mut buffer = Vec::with_capacity(1000);
    udp.recv(&mut buffer).unwrap();
    println!("{:?}", &buffer);
}

