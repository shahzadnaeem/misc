use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub fn one_shot_tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    for conn in listener.incoming() {
        match conn {
            Ok(mut stream) => {
                let mut data = [0u8; 100];
                stream.read(&mut data).unwrap();
                stream.write(b"Hello and bye\n").unwrap();
                break;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        }
    }
}
