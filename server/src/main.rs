use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let first_line: String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_path) = match first_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "server/hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "server/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "server/404.html"),
    };

    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
