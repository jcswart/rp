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

I want to try and read several bytes and print them rather than use string, because that requires closing the connection. After looking at the API docs this seems like a better first step:

```
use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {

    let server = TcpListener::bind("127.0.0.1:8765").unwrap();

    for stream in server.incoming() {
        match stream {
            Ok(mut stream2) => {
                for byte in stream2.bytes() {
                    println!("{:?}", byte.unwrap() as char)
                }
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }
}
```

The call to `bytes()` returns an iterator of bytes. The data is streamed from the socket and one character at a time is parsed.

This does bring up a potential problem that I will have to solve later: _concurrency_. Currently the `for stream in server.incoming()` is a blocking operation. Each TcpStream must be handled one at a time, this would not be ideal for a production server. The call to `bytes()` is similar in nature. At this point I don't really know how to even begin solving that in an idiomatic rust way
. A bit more googling and [this blog post seems like a good first step.](https://gkbrk.com/2016/08/asynchronous-servers-in-rust/)

Okay so `bytes()` isn't so great because inside the for loop I can't write to the stream.

Instead I refactored things to become:

```
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
```

The current server now:

1. Accepts a connection.
1. Attempts to read 1 byte at a time from the stream.
1. Telnet is line buffered so it gets several bytes at time, reads them and sends them back one at a time.

I even tested it with redis-cli, `$ redis-cli -p 8765` and its echo'ing back data nicely.

I think the simple echo server is solved and now its time to move on to building the RESP parser.
