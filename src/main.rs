mod interface;
mod parser;
mod percentage;

use std::env;
use interface::Interface;
use percentage::Percentage;

fn main() {
    let args: Vec<String> = env::args().collect();
    parser::check_help(&args);
    let interfaces = interface::get_interfaces();
    // get default interface
    // TODO check if interfaces[0] exists
    let selected_interface: &Interface = &interfaces[0];

    if args.len() >= 2 {
        parser::handle_args(&selected_interface, &args);
    } else {
        // without any args display the current state:
        println!("{}", &selected_interface.brightness());
    }
}
