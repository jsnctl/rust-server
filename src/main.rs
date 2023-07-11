use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

const HOST: &str = "127.0.0.1";
const PORT: &str = "8081";

fn main() {
    let addr = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(addr).unwrap();

    let mut count = 0;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        count += 1;

        println!("Total no. requests: {}", count)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request)
}
