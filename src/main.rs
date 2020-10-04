use std::net::TcpListener;
use std::thread;
use mpdrs::messages;

fn main() {
    // TODO: cmdline opts, ports, server, verbosity
    // Bind to port
    let listener = TcpListener::bind("127.0.0.1:6600").unwrap();
    println!("MPDRS running on port 6600");

    for stream in listener.incoming() {
        // Spawn worker thread
        thread::spawn(|| {
            println!("Client connected");
            messages::handle_client(stream.unwrap());
        });
    }
}
