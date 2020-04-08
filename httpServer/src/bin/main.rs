use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use rust_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }
    // for stream in listener.incoming().take(2) {
    //     let stream = stream.unwrap();

    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "pages/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "pages/hello.html")
    } else if buffer.starts_with(post) {
        // Add message to db
        store_message_textfile(buffer);
        ("HTTP/1.1 200 OK\r\n\r\n", "pages/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "pages/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn store_message_textfile(message: [u8; 512]){
    let text = String::from_utf8_lossy(&message[..]);
    let pos_body = text.find("user=");
    let important_text = &text[pos_body.unwrap()..512];

    let and_pos = important_text.find("&");
    let user = &important_text[5..and_pos.unwrap()];
    let message_text = &important_text[and_pos.unwrap()+9..512-pos_body.unwrap()];

    let mut file = File::create("messages.txt").expect("cant create File");
    // let mut file = if std::path::Path::new("messages.txt").exists() {
    //     // File::with_options().write(true).open("messages.txt").expect("Cant open File")
    // }else{
    //     File::create("messages.txt").expect("cant create File")
    // };
    let x = format!("{};{}", user, message_text);
    println!("{}", x);
    file.write_all(x.as_bytes()).expect("cant write to file");
}