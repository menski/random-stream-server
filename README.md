# Random Stream Server

A simple TCP streaming server with a custom binary message protocol.

## Usage

Pre-build release from the [release page](https://github.com/menski/random-stream-server/releases):
```
./random-stream-server [bind-address]
```

Docker:
```
docker run --init -p 9001:9001 menski/random-stream-server:latest
```

Cargo:
```
cargo run -- [bind-address]
```

The server will listen on the given `bind-address` or default to
`127.0.0.1:9001`. For every connection a thread is spawned to stream messages
to the client.

## Message Protocol

The server implements a simple binary message protocol over TCP with the
following framing.

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                            Length                             |
|                                                               |
+---------------------------------------------------------------+
|                                                               |
|                            Message                            |
...                                                           ...
+---------------------------------------------------------------+
```

- `Length`: the length of the following message as `uint64` in big-endian byte
  order
- `Message`: a UTF-8 encoded String with the length of the `Length` field

## Client/Server Protocol

The server binds to a TCP port and for every connecting client expects a
initial `hello` message. Which is a single message with the String `hello` as
content. The framing is described in the [message protocol
section](#message-protocl).

After the server received the initial `hello` message of the client it will
stream random messages to the client until the connection is closed.
