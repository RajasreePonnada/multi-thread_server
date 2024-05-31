// use std::net::TcpListener;
// use std::io::{Read, Write};

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//         println!("Connection established!");

//         let mut buffer = [0; 1024]; // Using a 1024 byte buffer
//         let bytes_read = stream.read(&mut buffer).unwrap();

//         if bytes_read > 0 {
//             let response = format!("Received {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);
//             stream.write_all(response.as_bytes()).unwrap();
//         }

//         stream.flush().unwrap(); // Flush the data to ensure it's sent
//         break; // Exit the loop after handling the first connection
//     }
// }

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
// --snip--

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}






