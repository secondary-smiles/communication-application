use crate::messages::types::{Body, Head, Kind, Message};
use crate::security::cert::Cert;

use toml;
use std::net::TcpStream;
use std::io::prelude::*;
// use toml::value::{Datetime, Date, Time, Offset};

pub fn init(mut stream: &mut TcpStream) -> bool {
    let mut buffer = [0; 2031];
    stream.read(&mut buffer).unwrap();
    check_cert(&buffer, &mut stream)
}

fn check_cert(cert_in: &[u8; 2031], stream: &mut TcpStream) -> bool {
    let cert_string = String::from_utf8_lossy(cert_in);
    let cert_string = cert_string.trim_end_matches(char::from(0));
    let cert: Cert = match toml::from_str(&cert_string) {
        Ok(v) => v,
        _ => {
            _warn_bad_cert(&cert_string.to_string(), "BAD CERT FORMAT".to_string(), stream);
            return false;
        }
    };

    if cert.verify() {
        return true;
    } else {
        _warn_bad_cert(&cert_string.to_string(), "BAD CERT HASH".to_string(), stream);
        return false;
    }
}

fn _warn_bad_cert(cert: &String, message: String, stream: &mut TcpStream) {
    eprintln!("WARNING BAD CERT\n{}\n{}", cert, message);

    let message: Message = Message {
        kind: Kind {
            kind: "CTXT".to_string(),
        },
        head: Head {
            sent: None,
            s_ID: "ME".to_string(),
            r_ID: "YOU".to_string(),
            size: 0,
            edited: Some(false),
            kind: "security/bad_cert".to_string(),
        },
        body: Body {
            data: Some(message),
            alert: None,
            w_TIME: None,
            m_ID: None,
            m_TO: None,
        },
    };

    let m_toml = toml::to_string(&message).unwrap();

    stream.write(m_toml.as_bytes()).unwrap();
}

// fn _now_to_datetime() -> Datetime {
//     THIS IS A WIP FUNCTION
//     Datetime {
//         date: Some(Date {
//             year: 2000,
//             month: 01,
//             day: 01,
//         }),
//         time: Some(Time {
//             hour: 0,
//             minute: 0,
//             second: 0,
//             nanosecond: 0,
//         }),
//         offset: Some(Offset::Z)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use crate::security::{cert, pem};

    #[test]
    fn test_init() {
        let listener = TcpListener::bind("0.0.0.0:4949").unwrap();
        let handle = thread::spawn(move || {
            let mut iter = 0;
            for stream in listener.incoming() {
                iter += 1;
                let mut stream = stream.unwrap();
                let res = init(&mut stream);
                stream.write(res.to_string().as_bytes()).unwrap();

                if iter >= 3 {
                    return;
                }
            }
        });

        let mut buffer = [0; 2031];
        let mut stream = TcpStream::connect("127.0.0.1:4949").unwrap();
        stream.write(b"bad test").unwrap();
        stream.read(&mut buffer).unwrap();
        let res = String::from_utf8_lossy(&buffer);
        let res = res.trim_end_matches(char::from(0));
        let r_i = res.rfind("false").unwrap();

        toml::from_str::<Message>(&res[..r_i]).unwrap();

        let mut stream = TcpStream::connect("127.0.0.1:4949").unwrap();
        let mut buffer = [0; 2031];
        let pem = pem::new();
        let cert = cert::new(&pem);
        stream.write(toml::to_string(&cert).unwrap().as_bytes()).unwrap();
        stream.read(&mut buffer).unwrap();
        let res = String::from_utf8_lossy(&buffer);
        let res = res.trim_end_matches(char::from(0));
        assert_eq!("true", res);

        let mut stream = TcpStream::connect("127.0.0.1:4949").unwrap();
        let mut buffer = [0; 2031];
        let pem = pem::new();
        let mut cert = cert::new(&pem);
        cert.signature = "Not a valid signature".to_string();
        stream.write(toml::to_string(&cert).unwrap().as_bytes()).unwrap();
        stream.read(&mut buffer).unwrap();
        let res = String::from_utf8_lossy(&buffer);
        let res = res.trim_end_matches(char::from(0));
        let r_i = res.rfind("false").unwrap();
        toml::from_str::<Message>(&res[..r_i]).unwrap();

        handle.join().unwrap();
    }
}
