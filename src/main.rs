use std::net::UdpSocket;

pub mod header;

fn main() {

    for _ in 0..10 {
        let message = header::SipMessage {
            start: header::StartLine::RequestLine(header::RequestLine {
                method: header::Method::Register,
                uri: header::RequestUri {},
                version: header::SipVersion::Sip2,
            }),
            headers: Vec::new(),
        };

        println!("SIP message: {}", message);
    }


    println!("Connecting to SIP server...");
    let udp = UdpSocket::bind("0.0.0.0:5061").unwrap();
    udp.connect("192.168.1.143:5060").unwrap();
    udp.send("REGISTER sip:192.168.1.143 SIP/2.0\n\n".as_bytes());
    let mut buffer = Vec::with_capacity(1000);
    udp.recv(&mut buffer);
    println!("{:?}", &buffer);
}

