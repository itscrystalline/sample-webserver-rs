use std::path::PathBuf;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}
    ,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap())
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Accepted Connection!");

    let buf = BufReader::new(&stream);
    let request: Vec<_> = buf
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_path: Vec<&str> = request[0].split(" ").collect();
    let path_string = format!("./html/{}", request_path[1]);

    let mut path = PathBuf::from(path_string);
    let mut status = String::new();

    let content: String = if (path.exists()) {
        if (path.is_dir()) {
            path.push("index.html");
        }
        match fs::read_to_string(path) {
            Ok(index) => {
                status = "200 OK".to_string();
                index
            }
            Err(err) => {
                status = "404 Not Found".to_string();
                fs::read_to_string("./html/404.html").unwrap()
            }
        }
    } else {
        status = "404 Not Found".to_string();
        fs::read_to_string("./html/404.html").unwrap()
    };

    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, content);

    stream.write_all(response.as_bytes()).unwrap();
}
