# Peer to Peer Gossip Simulator

This is a demo of what I can do in Rust. It's not meant to be a
complete application, but rather a showcase of coding style,
understanding of `async`, and knowledge of `actix_web` and `tokio`.

The same effect could have been accomplished with a lot more code, and
fewer dependencies, which could lead to better compilation time, but
not necessarily to better performance or better readability of said
code.

## Installation

This is a standard `rust` project.

Clone the codebase.

```bash
git clone https://github.com/appetrosyan/p2pgossip-example.git
```

then compile the release build.
```bash
cd p2pgossip-example
cargo build --release
```

After a bit of waiting you will have a statically linked `p2pgossip`
executable in `target/release/`. You are now a proud owner of a
`p2pgossip` executable. You may wish to `export PATH="$PATH:$(pwd)"`,
if you like, or move the executable to somewhere that is already in
your `PATH`. Since this is something I knocked out in a couple of
days, and it doesn't do much of anything useful, I wouldn't bother in
this particular case.

## Usage

An instance requires a port number to run. So usage is as follows:
```bash
p2pGossip
Aleksandr Petrosyan

USAGE:
	p2pgossip [FLAGS] [OPTIONS] --port <port>

FLAGS:
	-h, --help       Prints help information
	-u, --update     Whether to also fetch and update the set of known peers from other peers
	-V, --version    Prints version information

OPTIONS:
	-c, --connect <connect>...       The first peer to connect to [aliases: connect-to, make-connection]
	-a, --host-alias <host_alias>    The return address to be used by other peers. `localhost` by default.
	-P, --period <period>             [default: 5]  [aliases: message_interval, message_period]
	-p, --port <port>                The port to start the peer listening on
```


TODO

#### Port number `-p` `--port`

This is necessary to start the instance. If the port is not free, the
application exits immediately.

#### Peer addresses to connect to `-c` `--connect`

This is what initiates the gossip. The gossip message is "[random
message]". I contemplated using an NLP library and spinning up a
python interpreter inside my application, but this is also a "*random
message*".

- Peers connect to all sockets (`"{}:{}", IP_address, port`),
specified here that respond as instances of a `p2pgossip` program.

**Rationale**: You may be running other TCP listeners on the port you
specified accidentally. We don't want to spam that application with
periodic messages. We need to explicitly establish the connection.

- Peers connect reflexively. This means that if peer A connects to B,
  B also connects to A.


#### period `-P`, `--period`


Specified in seconds is the period with which your current instance of
a peer is going to message the other peers.

#### update `-u`, `--update`

By default peers only connect to the peers that they were given at the
CLI, and peers that connected to them explicitly. This can be changed
to retrieve all connected peers with the same periodicity, as
messaging. If `-u** is passed when the server is started, then this
particular peer will fetch the list of known peers from the peers it's
already connected to. The peers that it is connected to, however, will
not do the same.

**Rationale**: The peers can grow uncontrollably. Ideally there should
be a mechanism that allows password-based authentication to limit the
growth, but I ran out of time.


#### Host alias.

Provide the listening ipAddress to which the peer is to connect.

**Rationale**: In theory this application could work on more than your localhost. If
you know how to connect to it, via either assigning one of the peer
instances a domain name, or an IP address other than 127.0.0.1, you
should be able to communicate across networks.

## TODO

- [ ] Unit Tests.
- [ ] Integration tests.
- [ ] profiling.
- [ ] communicating over https with TLS encryption.
- [ ] allowing peers to adjust the runtime parameters of this peer.
- [ ] using an atomic concurrent HashMap.
