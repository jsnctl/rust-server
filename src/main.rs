use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const HOST: &str = "0.0.0.0";
const PORT: &str = "8081";

fn main() {
    println!("Server is up...");
    let addr = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take(3)
        .collect();

    let http_version = "HTTP/1.1";
    let ok = "200 OK";
    let not_found = "404 NOT FOUND";

    let request_path = http_request
        .first()
        .unwrap()
        .strip_suffix(" HTTP/1.1")
        .unwrap();

    let (status, doc) =
        if request_path == "GET /" {
            (format!("{http_version} {ok}"), "assets/index.html")
        } else {
            (format!("{http_version} {not_found}"), "assets/404.html")
        };

    let contents = fs::read_to_string(doc).unwrap();
    let length = contents.len();

    println!("{:#?}", http_request);


    let response = format!(
        "{status}\r\n\
         Content-Length: {length}\r\n\r\n\
         {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
