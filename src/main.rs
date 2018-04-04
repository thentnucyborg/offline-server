extern crate byteorder;
extern crate bytes;
extern crate csv;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate tokio_io;

use controller::Controller;
use std::{fs, thread};

mod tcp;
mod http;
mod controller;

fn main() {
    let mut args = std::env::args();
    let mode = args.nth(1).unwrap();

    let controller_input = match mode.as_str() {
        "run" => None,
        "build" => Some(args.next().expect("No filename given.")),
        "clear" => {
            for i in 0..60 {
                let _ = fs::remove_file(format!(".{}.dat", i));
            }

            std::process::exit(0);
        },
        "help" => {
            println!(r#"Available commands:
    build <filename>.csv - Builds the cache using the supplied CSV file and runs the server.
    run - Runs the server using the files built using "build <filename>.csv"
    clear - Clears the cached files.
    help - Shows this help information."#);
            std::process::exit(0);
        }
        _ => {
            eprintln!("Invalid arguments. Try running with help.");
            std::process::exit(1);
        },
    };


    // Create a thread handle vector on which to let main join:
    let mut threads = Vec::new();

    // Create channels for communication between HTTP server and MEA read thread:
    let (command_tx, command_rx) = std::sync::mpsc::channel();

    let http_addr = "0.0.0.0:1234".parse().unwrap();
    threads.push(thread::spawn(move || {
        http::Server::new().run(&http_addr, command_tx);
    }));

    let tcp_addr = "0.0.0.0:12345".parse().unwrap();
    let tcp_server = tcp::Server::bind(&tcp_addr);
    let clients = tcp_server.get_clients();

    threads.push(thread::spawn(move || {
        tcp_server.run();
    }));

    threads.push(thread::spawn(move || {
        let controller = Controller::new(command_rx, clients, controller_input);
        controller.run();
    }));


    // Finally join all server threads:
    for i in threads {
        i.join().unwrap();
    }
}