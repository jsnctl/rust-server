use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use rust_server::ThreadPool;

const HOST: &str = "0.0.0.0";
const PORT: &str = "8081";

struct Server {
    addr: String,
    listener: TcpListener,
    pool: ThreadPool,
}

impl Server {
    fn new() -> Server {
        let pool = ThreadPool::new(5);
        let addr = format!("{}:{}", HOST, PORT);
        println!("Server is up...");
        Server {
            addr: addr.to_owned(),
            listener: TcpListener::bind(addr).unwrap(),
            pool,
        }
    }
}

fn main() {
    let server = Server::new();
    for stream in server.listener.incoming() {
        let stream = stream.unwrap();

        server.pool.execute(|| {
            handle_connection(stream);
        });
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

    let (status, doc) = match &request_path[..] {
        "GET /" => (format!("{http_version} {ok}"), "assets/index.html"),
        "GET /slow" => {
            thread::sleep(Duration::from_secs(10));
            (format!("{http_version} {ok}"), "assets/index.html")
        }
        _ => (format!("{http_version} {not_found}"), "assets/404.html")
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
