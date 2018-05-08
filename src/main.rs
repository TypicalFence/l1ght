mod interface;
mod parser;
mod percentage;

use std::env;
use std::ops::Index;
use interface::Interface;


fn get_default_interface() -> Option<Interface> {
    let interfaces = interface::get_interfaces();

    if  interfaces.len() < 1 {
        // TODO some output to the user
        std::process::exit(1);
    }
    let default_interface: Interface = interfaces[0].clone();
    Some(default_interface)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    parser::check_help(&args);

    if args.len() >= 2 {
        if args.contains(&String::from("-i")) {
            // this is clone().clone() is wird desu
            let position = args.iter().position(|ref x| x.clone().clone() == String::from("-i")).unwrap();
            let interface = interface::get_interface(args.index(position + 1));

            match interface {
                Some(interface)  => {
                    // is last arg
                    if position + 1 == args.len() - 1 {
                        println!("{}", &interface.brightness());
                    } else {
                        parser::handle_args(&interface, &args);
                    }
                },
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
