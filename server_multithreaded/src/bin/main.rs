use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use server_multithreaded::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    

    let pool = ThreadPool::new(4); //need to implement thread pool

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //handle_connection(stream);
        //println!("Connection Established"); //printed several times because connection is tried multiplte times waiting for output
        pool.execute( || {
            handle_connection(stream); //instead of using unlimited threads, use those limited threads in the pool to call for this 
        });
        /*thread::spawn( || {
            handle_connection(stream); //closure, which creates new thread everytime and handles new connections
            //other method is to create a thread pool
        });*/
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n"; //hard code request line we are expecting 
    //prefixing it with b gives us a byte array representing our string
    let sleep = b"GET /sleep HTTP/1.1\r\n";


    let (status_line, filename) =
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK","index.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(4));
            ("HTTP/1.1 200 OK","index.html")
        }   else {
            ("HTTP/1.1 404 NOT FOUND","404.html")
        };


    let contents = fs::read_to_string(filename).unwrap();

    //let response = "HTTP/1.1 200 OK\r\n\r\n";

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(), //to pass output size along with our output to get number of bytes we need to return
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();//wait until bytes and written to response
    //this helps return a blank page as output -> then we add content and change response using a format macro
    println!("Request: {}",String::from_utf8_lossy(&buffer[..]));//convert slice of buffer into a string

    //HTTP-Version Status-code Reason-phrase CRLF
    //headers CRLF
    //message-body
    //
    //ex: HTTP/1.1 200 OK\r\n\r\n - carriage return line feed sequence

}
