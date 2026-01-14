use std::fs;
use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Let's start our server by creating a listener for a local addres.
    // `bind` is like `new` in networking words.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // The incomming method return an iterator with the *stremas*, the streams
    // are open connections between client and the server
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection_v2(stream);
    }
}

// Now create a connection handler to read the data sent from the browser.
fn handle_connection(mut stream: TcpStream) {
    // Read the data with `BufReader`, it's faster than simple read. It is used
    // when the data is short and fast.
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap()) // Stop the program if data isn't valid UTF-8
        // or cannot read.
        .take_while(|line| !line.is_empty()) // The browsers end the signals when
        // sends two empty lines.
        .collect();
    // We add some logic to only answer if requesting to `/` and error anything else.
    let request_line = &http_request[0];
    if request_line == "GET / HTTP/1.1" {
        // Now, we add a response. Responses have the format.
        // ```
        // HTTP-Version Status-Code Reason-Phrase CRLF
        // headers CRLF
        // message-body
        // ```
        // Blank page without errors.
        // let response = "HTTP/1.1 200 OK\r\n\r\n";

        // Instead of hardcoding the response, we use a html file.
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("exercises/23_hello_web_server/hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // Send a response with status code 404, which signals the content requested
        // was not found.
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("exercises/23_hello_web_server/404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

// Now, with some refactoring to acoid repetition and be more concise.
fn handle_connection_v2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        (
            "HTTP/1.1 200 OK",
            "exercises/23_hello_web_server/hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            "exercises/23_hello_web_server/404.html",
        )
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
