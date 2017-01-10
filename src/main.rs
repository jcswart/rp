use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {

    let server = TcpListener::bind("127.0.0.1:8765").unwrap();

    for stream in server.incoming() {
        match stream {
            Ok(mut stream2) => {
                let mut buf = String::new();
                let bytes_read = stream2.read_to_string(&mut buf);
                println!("stream: {:?}", stream2);
                println!("got a bite! {:?}", buf);
                println!("bytes read {:?}", bytes_read);
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }
}
