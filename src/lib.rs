/// Contains functions for receiving, parsing, and sending messages over the connection
pub mod messages {
    use std::io::{Read, Write};
    use std::net::{TcpStream};
    use super::stdout;

    /// Handles messages
    pub fn handle_client(mut stream: TcpStream) {
        // All data between client and server is UTF-8 encoded
        let mut data = [0; 1024];

        // MPD client connection response
        send(&mut stream, "OK MPD 0.21.11\n");
        stdout::print_mpdrs("Connection established");

        // Interpret commands
        let mut cmd_list_mode = false;
        stream.read(&mut data).unwrap();

        for line in String::from_utf8_lossy(&data[..]).lines() {
            stdout::print_client(format!("'{}'", line).as_str());

            // Switch to another handler if in a command list block
            if cmd_list_mode {
                cmd_list_mode = handle_command(line);
                send(&mut stream, "OK\n")

            } else {
                match line {
                    // Enter or exit command list block
                    "command_list_ok_begin" => {
                        stdout::print_mpdrs("Entering command mode");
                        cmd_list_mode = true
                    },
                    "command_list_end" => {
                        // this probably shouldn't exist
                        stdout::print_mpdrs("Exiting command mode from client handler");
                        cmd_list_mode = false;
                    },

                    // Everything else
                    _ => stdout::print_mpdrs("Unhandled message received."),
                }
            }
        }
    }

    /// Handles command, returns true if program should remain in command mode
    fn handle_command(command: &str) -> bool {
        match command {
            // Exit command list mode
            "command_list_end" => {
                stdout::print_mpdrs("Exiting command mode");
                return false
            },
            // "status" => println!("GOT STATUS COMMAND"),
            _ => {
                stdout::print_mpdrs("Unhandled command received.");
                return true
            }
        }
    }

    /// Sends a message
    fn send(stream: &mut TcpStream, message: &str) {
        stdout::print_mpdrs(format!("'{}'", message).as_str());
        stream.write(message.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

/// Standard output utilities
pub mod stdout {
    use termion::{color, style};

    /// Prints a server-related message in a nice format
    pub fn print_mpdrs(message: &str) {
        println!("[{bold}{green}MPDRS{reset}]: {message}",
            bold = style::Bold,
            green = color::Fg(color::Magenta),
            reset = style::Reset,
            message = message.replace("\n", ""));
    }

    /// Prints a client-related message in a nice format
    pub fn print_client(message: &str) {
        println!("[{bold}{green}CLIENT{reset}]: {message}",
            bold = style::Bold,
            green = color::Fg(color::Cyan),
            reset = style::Reset,
            message = message.replace("\n", ""));
    }
}
