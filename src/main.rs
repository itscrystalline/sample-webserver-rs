use core::error;
use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, str::FromStr};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap())
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Accepted Connection!");
    
    let buf = BufReader::new(&stream);
    let request: Vec<_> = buf.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_path: Vec<&str> = request[0].split(" ").collect();
    let path = format!(".{}", request_path[1]);

    let mut status = String::new();
    let file_request = fs::read_to_string(path);
    let file = match file_request {
        Ok(content) => content,
        Err(err) => panic!("{err:?}")
    };
    println!("{}", file);

    let template = "HTTP/1.1 200 OK\r\n\r\nhai!!!!!!!!!!!!!!!!!!!!";

    stream.write_all(template.as_bytes()).unwrap();
}
