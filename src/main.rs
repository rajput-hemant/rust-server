use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // a listener is a struct that listens for incoming TCP connections
    let listner = TcpListener::bind("127.0.0.1:3000").unwrap();

    // incoming() returns an iterator over connections to the listener
    for stream in listner.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!")
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // create a buffer to hold the data(requests) from the stream
    // in this case, we're taking a buffer of 1024 bytes
    // however in a production application, you would want to
    // use a buffer of arbitrary size
    let mut buffer = [0; 1024];

    // read the data from the stream into the buffer
    // or populate the buffer with the data from the stream
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // check if the request is a GET request,
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // read the contents of the index.html file into a string
    let contents = fs::read_to_string(filename).unwrap();

    // let response = "HTTP/1.1 200 OK\r\n\r\n"; // \r\n is CRLF(Carriage Return Line Feed)

    let response = format!(
        "{}r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // write the response to the stream
    stream.write(response.as_bytes()).unwrap();
    // flush the stream, which will actually send the response to the client
    stream.flush().unwrap();
}
