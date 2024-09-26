mod interface;

use std::num::ParseIntError;
use std::process::ExitCode;
use crate::interface::Interface;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Args {
    display_percentage: bool,
    interface: Option<String>,
    action: Option<String>
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse Args: {}.", e);
            std::process::exit(1);
        }
    }; 

    let interface_name = match args.interface.or_else(|| get_default_interface().map(|x| x.get_name())) {
        Some(i) => i,
        None => {
            eprintln!("No backlight devices available on this computer!");
            std::process::exit(1);
        }
    };

    let interface = match interface::get_interface(&interface_name) {
        Ok(i) => i,
        Err(_) => {
            eprintln!("No backlight devices available on this computer!");
            std::process::exit(1);
        }
    };

    let max= interface.get_max();

    if let Some(action) = args.action {
        let number = get_number_from_action(&action);

        if number.is_err() {
            eprintln!("Invalid non-numeric action: {}", number.err().unwrap());
            std::process::exit(1);
        }

        let value = match action.ends_with("%") {
            false => {number.unwrap()},
            true => {
                let percentage = number.unwrap();
                calculate_value_from_percentage(max, percentage)
            }
        };

        if action.starts_with("+") {
            interface.increase_brightness(value);
        } else if action.starts_with("-") {
            interface.decrease_brightness(value);
        } else {
            eprintln!("Invalid action: {}", action);
        }
    }

    let current = interface.brightness();

    if args.display_percentage {
        println!("{}%", calculate_percentage(max, current));
    } else {
        println!("{}", current);
    }

    ExitCode::SUCCESS
}

fn get_default_interface() -> Option<Interface> {
    let interfaces = interface::get_interfaces().unwrap_or_default();

    if  interfaces.is_empty() {
        return None;
    }

    let default_interface: Interface = interfaces[0].clone();
    Some(default_interface)
}

fn get_number_from_action(action: &str) -> Result<i32, ParseIntError> {
    let mut chars= action.chars();

    let mut peekable = chars.clone().peekable();
    let first_char = peekable.peek();
    if first_char == Some(&'+') || first_char == Some(&'-') {
        chars.next();
        
    }

    let num_str = chars.as_str();

    if num_str.ends_with("%") {
        let clean = num_str.strip_suffix(r"%").unwrap();
        return get_number_from_action(clean);
    }

    num_str.parse::<i32>()
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print_help();
        std::process::exit(0);
    }

    if pargs.contains(["-V", "--version"]) {
        println!("l1ght {}", VERSION);
        std::process::exit(0);
    }

    let args = Args {
        display_percentage: pargs.contains("-p"),
        interface: pargs.opt_value_from_str("-i")?.or_else(|| pargs.opt_value_from_str("--interface").unwrap_or(None)),
        action: pargs.opt_free_from_str()?,
    };

    
    let _ = pargs.finish();

    Ok(args)
}

#[inline]
fn calculate_percentage(total: i32, value: i32) ->  i32 {
    (value as f32 / total as f32 * 100.0) as i32
}

#[inline]
fn calculate_value_from_percentage(total: i32, percentage: i32) -> i32 {
    let value = total as f32 / 100.0 * percentage as f32;
    value as i32
}



#[inline]
pub fn print_help() {
    println!(
            "l1ght {}
fence <fence@desu-mail.moe>
A small cli for changing the backlight on a laptop.

USAGE:
    l1ght [FLAGS] [OPTIONS] ACTION

FLAGS:
    -h, --help       Prints this message.
    -V, --version    Prints the version.
    -p               Prints the current brightness value as a percentage.

OPTIONS:
    -i, --interface  Set a specific interface.

ACTIONS:
    nothing          Returns the current brightness value.
    +value           Increases the current brightness value.
    -value           Decreases the current brightness value.
    +percentage%     Increases the current brightness value by a percentage.
    -percentage%     Decreases the current brightness value by a percentage.

EXAMPLES:
    l1ght +50        Increases the current brightness value by 50.
    l1ght -5%        decreases the current brightness value by 5%",
           VERSION
        );
}