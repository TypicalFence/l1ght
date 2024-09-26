mod interface;
mod parser;

use std::env;
use std::ops::Index;
use crate::interface::Interface;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_default_interface() -> Option<Interface> {
    let interfaces = interface::get_interfaces().unwrap_or(Vec::new());

    if  interfaces.len() < 1 {
        println!("no backlight interface found");
        std::process::exit(1);
    }

    let default_interface: Interface = interfaces[0].clone();
    Some(default_interface)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    parser::check_help(&args, VERSION);
    parser::check_version(&args, VERSION);

    if args.len() >= 2 {
        if args.contains(&String::from("-i")) || args.contains(&String::from("--interface")) {
            let position = args.iter().position(|ref x| x.as_str() == "-i").unwrap();
            let interface = interface::get_interface(args.index(position + 1));

            match interface {
                Some(Ok(interface))  => {
                    if position == args.len() {
                        println!("{}", &interface.brightness());
                    } else {
                        parser::handle_args(&interface, &args);
                    }
                },
                Some(Err(_)) => {
                    println!("Failed to open interface");
                    std::process::exit(1);
                }
                None => {
                    println!("Unknown Interface");
                    std::process::exit(1);
                }
            }
        } else {
            let interface = get_default_interface().unwrap();
            parser::handle_args(&interface, &args);
        }
    } else {
        let interface = get_default_interface().unwrap();
        println!("{}", &interface.brightness());
    }
}
