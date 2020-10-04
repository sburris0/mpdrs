/// Contains functions for receiving, parsing, and sending messages over the connection
pub mod messages {
    use std::io::{Read, Write};
    use std::net::{TcpStream};

    /// Handles messages
    pub fn handle_client(mut stream: TcpStream) {
        // All data between client and server is UTF-8 encoded
        let mut data = [0; 1024];

        // MPD client connection response
        send(&mut stream, "OK MPD 0.21.11\n");
        println!("Connection established");

        // Interpret commands
        let mut cmd_list_mode = false;
        stream.read(&mut data).unwrap();

        for line in String::from_utf8_lossy(&data[..]).lines() {
            // Switch to another handler if in a command list block
            if cmd_list_mode {
                cmd_list_mode = handle_command(line);
                println!("OK");
                send(&mut stream, "OK\n")

            } else {
                match line {
                    // Enter or exit command list block
                    "command_list_ok_begin" => {
                        println!("MESSAGE: '{}', Entering command mode", line);
                        cmd_list_mode = true
                    },
                    "command_list_end" => {
                        println!("MESSAGE: '{}', Exiting command mode from client handler", line);
                        cmd_list_mode = false;
                    },

                    // Everything else
                    _ => println!("MESSAGE: '{}'", line),
                }
            }
        }
    }

    /// Handles command, returns true if program should remain in command mode
    fn handle_command(command: &str) -> bool {
        match command {
            // Exit command list mode
            "command_list_end" => {
                println!("COMMAND: '{}', Exiting command mode", command);
                return false
            },
            // "status" => println!("GOT STATUS COMMAND"),
            _ => {
                println!("COMMAND: '{}'", command);
                return true
            }
        }
    }

    /// Sends a message
    fn send(stream: &mut TcpStream, message: &str) {
        stream.write(message.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
