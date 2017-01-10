+++
title = "First Steps"
date = "2017-01-09T19:38:38-05:00"

+++

## Project setup

This blog lives inside the codebase for the time being. Projects are made with the cargo program.

`$ cargo new --bin redis-proxy`

I then used the __hugo__ static blogging tool to create a blog in the _redis-proxy/blog_ directory.

We can now compile the hello world app with `$ cargo run`.

```bash
$  cargo run
   Compiling redis-proxy v0.1.0 (file://redis-proxy)
    Finished debug [unoptimized + debuginfo] target(s) in 0.39 secs
     Running `target/debug/redis-proxy`
Hello, world!
```

## RESP: Redis Protocol

Redis clients communicate to the redis server via RESP.

A simple example looks like this:

```text
*2
$3
GET
$5
mykey
42
```

We won't get in to the specifics yet but at a high level the protocol is saying:

```text
*2    - there are two arguments
$3    - the next argument length is 3 characters
GET   - the first argument, the GET command
$5    - the next argument is 5 characters
mykey - the second argument, the key
42    - the response
```

## Sockets

I'm personally less interested in beginning with writing the RESP parser because I've written a number of simple parsers in rust at this point. I haven't done any work with sockets; however, so I'll likely start there by making a simple echo server.
