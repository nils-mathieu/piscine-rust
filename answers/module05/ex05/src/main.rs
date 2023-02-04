use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let (addr, location) = arg.split_once('/').unwrap_or((arg.as_str(), ""));
    let mut stream = TcpStream::connect((addr, 80)).unwrap();

    writeln!(
        stream,
        "\
        GET /{location} HTTP/1.1\r\n\
        Host: {addr}\r\n\
        Connection: close\r\n\
        \r\n\
        "
    )
    .unwrap();

    let mut stdout = std::io::stdout();
    let mut buffer = [0u8; 4096];
    loop {
        let count = stream.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }

        stdout.write_all(&buffer[..count]).unwrap();
    }
}
