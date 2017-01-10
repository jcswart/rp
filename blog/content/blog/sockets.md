+++
date = "2017-01-09T20:18:58-05:00"
title = "Sockets"

+++

## String echo server

To build the socket server a quick google lead to: https://doc.rust-lang.org/std/net/struct.TcpListener.html

I'll start by copy the example and seeing if that will be enough.

My modified example:

```rust
use std::net::{TcpListener, TcpStream};

fn main() {

    let server = TcpListener::bind("127.0.0.1:8765").unwrap();

    for stream in server.incoming() {
        match stream {
            Ok(stream2) => {
                println!("got a bite! {:?}", stream2);
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }
}
```

I ran the server with `$ cargo run` and ~In another terminal I connected and saw:

```
$ telnet 127.0.0.1 8765
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
Connection closed by foreign host.
```

And back in the terminal running the server:

```
got a bite! TcpStream { addr: V4(127.0.0.1:8765), peer: V4(127.0.0.1:57102), fd: 4 }
```

So it seems that either the println! or the for/match closed our socket before we could do anything. Looking at the docs for TcpStream https://doc.rust-lang.org/std/net/struct.TcpStream.html shows a few things that we might do.

Lets try to read the stream into a string, then write it back.

This is what I came up with.

```
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
```

I then attempted to connect via telnet and send.

```
$ telnet 127.0.0.1 8765
abc^]
telnet> q
Connection closed.
```

The server terminal process would show:

```
got a bite! ""
```

After much googling and finally asking for help in #rust-beginners I learned that _telnet_ starts in linemode, where it reads line by line. This means that I needed a `\r\n` after my `abc` in order to send any data. Once I learned this I was able to see the string print.

## Bytes.

I want to try and read several bytes and print them rather than use string, because that requires closing the connection.
