use std::str;
use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8989").expect("Cannot connect to specified address");

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        stream.write(input.as_bytes()).expect("Cannot write input string int outgounf tcp channel");

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        println!("{}", str::from_utf8(&buffer).expect("Could not wqrite buffer into a string"));
    }
}