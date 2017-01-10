use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {

    let server = TcpListener::bind("127.0.0.1:8765").unwrap();

    for stream in server.incoming() {
        match stream {
            Ok(mut stream2) => {
                let mut buf: [u8; 1] = [0; 1];
                loop {
                    let read = stream2.read_exact(&mut buf);
                    match read {
                        Ok(_) => {
                            println!("{:?}", buf[0] as char);
                            stream2.write(&buf);
                        }
                        Err(e) => {
                            println!("err: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }
}
