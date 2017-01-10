+++
date = "2017-01-09T19:15:56-05:00"
title = "Introduction"
draft = true

+++

The following blog is about learning enough of the Rust programming language to implement a Redis Proxy.

We want to:

* create a server that accepts TCP socket connections
* accepts RESP, the redis protocol
* outputs RESP on an outbound socket

I recently assisted in a similar project in Clojure and I want to attempt the same in Rust.
