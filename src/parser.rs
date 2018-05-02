use std;
use interface::Interface;

pub enum NumPos {
    Prepended,
    Appended,
}

pub fn check_help(args: &Vec<String>) {
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("*please insert help message here*");
        std::process::exit(0);
    }
}

pub fn get_number(action: &String) -> Option<i16> {
    let action = action.clone();
    let mut chars = action.chars();
    // skip first char (opperator)
    chars.next();
    let num = chars.as_str().parse::<i16>();
    if num.is_ok() {
        return Some(num.unwrap());
    }
    None
}

pub fn handle_args(interface: &Interface, args: &Vec<String>) -> bool {
    let action: &String = args.last().unwrap();

    if action.starts_with("+") {
        let mut number = get_number(action);
        if number.is_some() {
            &interface.increase_brightness(number.unwrap());
            return true;
        }
    }

    if action.starts_with("-") {
        let mut number = get_number(action);
        if number.is_some() {
            &interface.decrease_brightness(number.unwrap());
            return true;
        }
    }

    // TODO check if number or percentage

    false
}
