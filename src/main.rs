extern crate ascii;

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

use ascii::AsciiString;

type String = AsciiString;
type Username = AsciiString;

struct Server {
    users: HashMap<Username, User>,
}

struct Channel {
    users: HashMap<Username, User>,
}

struct User {
    id: String,
    nickname: String,
    server: String,
    host: String,
    hostname: String,
    connection: TcpStream,
}

struct Service {
    id: String,
    server: String,
    nickname: String,
    connection: TcpStream,
}


fn main() {
    let addr = "127.0.0.1";
    let port = 6667;
    let socket = TcpListener::bind(&format!("{}:{}", addr, port)).unwrap();

    for conn in socket.incoming() {}
}
