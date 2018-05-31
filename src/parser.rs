use std;
use interface::Interface;
use percentage::Percentage;

enum ParseError {
    IsPercentage,
    IsInvalid,
}

pub fn check_help(args: &Vec<String>, version: &str) {
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
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
            version
        );
        std::process::exit(0);
    }
}

pub fn check_version(args: &Vec<String>, version: &str) {
    if args.contains(&String::from("--version")) || args.contains(&String::from("-V")) {
        println!("l1ght {}", version);
        std::process::exit(0);
    }
}


fn has_opperator(action: &str) -> bool {
    action.starts_with("+") || action.starts_with("+")
}


fn is_percentage(num: &str) -> bool {
    num.ends_with("%")
}

fn get_percentage(action: &str) -> Option<i8> {
    let action = action.clone();
    let mut chars = action.chars();
    // skip first char (opperator)
    chars.next();
    // remove last character (%)
    let mut action_string = chars.as_str().to_string();
    action_string.pop();

    let num_str = action_string.as_str();
    let num = num_str.parse::<i8>();

    if num.is_ok() {
        return Some(num.unwrap());
    }

    None
}

fn get_number(action: &String) -> Result<i32, ParseError> {
    let action = action.clone();
    let mut chars = action.chars();
    // skip first char (opperator)
    chars.next();
    let num_str = chars.as_str();

    if is_percentage(&num_str) {
        return Err(ParseError::IsPercentage);
    }

    let num = num_str.parse::<i32>();
    if num.is_ok() {
        return Ok(num.unwrap());
    }

    Err(ParseError::IsInvalid)
}

fn interpret_action(interface: &Interface, action: &str, num: i32) -> bool {
    if action.starts_with("+") {
        &interface.increase_brightness(num);
        return true;
    }

    if action.starts_with("-") {
        &interface.decrease_brightness(num);
        return true;
    }

    false
}

fn change_state(interface: &Interface, args: &Vec<String>) -> bool {
    let action: &String = args.last().unwrap();
    let number = get_number(action);

    match number {
        Ok(num) => {
            return interpret_action(&interface, &action, num);
        }
        Err(error) => {
            match error {
                ParseError::IsPercentage => {
                    let num = get_percentage(action);
                    if num.is_some() {
                        let percentage = Percentage::from_total_and_percentage(
                            interface.get_max().clone(),
                            num.unwrap(),
                        );
                        return interpret_action(&interface, &action, percentage.value);
                    } else {
                        return false;
                    }
                }
                ParseError::IsInvalid => {
                    return false;
                }
            }
        }
    }
}


pub fn handle_args(interface: &Interface, args: &Vec<String>) -> bool {
    let argument: &String = args.last().unwrap();
    if argument == "-p" {
        let max = interface.get_max().clone();
        let percentage = Percentage::from_total_and_value(max, interface.brightness());
        println!("{}", percentage.percentage);
        return true;
    }

    // no known argument was given
    // try to change the state with the argument given
    return change_state(&interface, args);
}
