extern crate tokio;

use std::env;
use std::io;

mod lib;

fn main() {
    env::set_var("RUST_LOG", "info");

    let mut command = String::new();

    match io::stdin().read_line(&mut command) {
        Ok(n) => match command.as_ref() {
            "-start\n" => {
                println!("SLPP started with params: {}", "{ some params }");
                lib::engine::client::client_handler();
            }
            "-create_channel\n" => {
                println!("Channel with {} created", "{ partner's name }");
            }
            "-close_channel\n" => {
                println!("Channel {} closed", "{ partner's name }");
            }
            "-make_node\n" => {
                println!("Node created!");
            }
            "-exit\n" => {
                println!("Exit from SLPP");
            }
            _ => {
                println!("Unknown command");
            }
        },

        Err(error) => println!("error: {}", error),
    }
}
