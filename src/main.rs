use std::net::TcpListener;
use std::thread;
use clap::{App, Arg};
use mpdrs::messages;

fn main() {
    let args = App::new("MPDrs")
        .version("0.1.0")
        .about("MPD-compatible music daemon written in Rust")
        .author("Spencer Burris <sburris@posteo.com>")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Port to run the daemon on (6600 by default)")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .help("Enables verbose output (useful for debugging)")) 
        .get_matches();

    // Bind to port
    let port = args.value_of("port").unwrap_or("6600");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("MPDRS running on port {}", port);

    for stream in listener.incoming() {
        // Spawn worker thread
        thread::spawn(|| {
            println!("Client connected");
            messages::handle_client(stream.unwrap());
        });
    }
}
