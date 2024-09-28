mod backlight;

use std::num::ParseIntError;
use std::process::ExitCode;

use backlight::DeviceId;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Args {
    display_percentage: bool,
    monitor: bool,
    list_devices: bool,
    output_max: bool,
    device: Option<String>,
    action: Option<String>,
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse Args: {}.", e);
            std::process::exit(1);
        }
    };


    // Listing all devices and returning early.
    if args.list_devices {
        let devices = backlight::list_devices().unwrap_or_default();

        for device in devices {
            println!("{}", device.0);
        }

        return ExitCode::SUCCESS;
    }
    

    let device_name = match args
        .device
        .or_else(|| get_default_device().map(|x| x.0))
    {
        Some(i) => i,
        None => {
            eprintln!("No backlight devices available on this computer!");
            std::process::exit(1);
        }
    };

    let device = match backlight::open_device(backlight::DeviceId(device_name.clone())) {
        Ok(i) => i,
        Err(_) => {
            eprintln!("No backlight devices available on this computer!");
            std::process::exit(1);
        }
    };

    // Printing the maximum brightness value and returning early.
    if args.output_max {
        let max = device.get_max_brightness();

        println!("{}", max);
        return ExitCode::SUCCESS;
    }

    if let Some(action) = args.action {
        let number = get_number_from_action(&action);

        if number.is_err() {
            eprintln!("Invalid non-numeric action: {}", number.err().unwrap());
            std::process::exit(1);
        }

        let value = match action.ends_with("%") {
            false => number.unwrap(),
            true => {
                let percentage = number.unwrap();
                calculate_value_from_percentage(device.get_max_brightness(), percentage)
            }
        };

        if action.starts_with("+") {
            if let Err(_) =  device.increase_brightness(value) {
                eprintln!("Failed to increase brightness.");
                return ExitCode::FAILURE;
            }
        } else if action.starts_with("-") {
            if let Err(_) = device.decrease_brightness(value) {
                eprintln!("Failed to decrease brightness.");
                return ExitCode::FAILURE;
            }
        } else if action.starts_with("=") {
            if let Err(_) = device.set_brightness(value) {
                eprintln!("Failed to set brightness.");
                return ExitCode::FAILURE;
            }
        } else {
            eprintln!("Invalid action: {}", action);
            return ExitCode::FAILURE;
        }
    }


    if args.monitor {
        return monitor_brightness(&device, args.display_percentage);
    }

    print_current_brightness(&device, args.display_percentage);
    ExitCode::SUCCESS

}

fn print_current_brightness(device: &backlight::Backlight, display_percentage: bool) -> bool {
    let current = device.get_actual_brightness();

    if current.is_err() {
        eprintln!("Failed to get current brightness: {}", current.err().unwrap());
        return false;
    }

    if display_percentage {
        println!("{}%", calculate_percentage(device.get_max_brightness(), current.unwrap()));
    } else {
        println!("{}", current.unwrap());
    }

    return true;
}

#[cfg(feature = "udev")]
fn monitor_brightness(device: &backlight::Backlight, display_percentage: bool) -> ExitCode {
    eprintln!("Monitoring brightness changes. Press Ctrl+C to exit.");

    print_current_brightness(&device, display_percentage);

    if let Ok(socket) = device.monitor() {
        let device_name = device.get_name();
        let mut iter = socket.iter();
        loop {
            iter.next().map(|event| {
                match  event.event_type() {
                    udev::EventType::Change => {
                        if event.device().sysname().to_string_lossy() == device_name {
                            print_current_brightness(&device, display_percentage);
                        }
                    },
                    _ => {}
                } 
            });
        }
    }

    ExitCode::SUCCESS
}

#[cfg(not(feature = "udev"))]
fn monitor_brightness(_device: &backlight::Backlight, _display_percentage: bool) -> ExitCode {
    eprintln!("Monitoring requires udev.");
    ExitCode::FAILURE
}


fn get_default_device() -> Option<DeviceId> {
    let devices = backlight::list_devices().unwrap_or_default();

    if devices.is_empty() {
        return None;
    }

    Some(devices[0].clone())
}

fn get_number_from_action(action: &str) -> Result<i32, ParseIntError> {
    let mut chars = action.chars();

    let mut peekable = chars.clone().peekable();
    let first_char = peekable.peek();
    if first_char == Some(&'+') || first_char == Some(&'-') || first_char == Some(&'=') {
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
        monitor: pargs.contains(["-M", "--monitor"]),
        list_devices: pargs.contains(["-l", "--list"]),
        output_max: pargs.contains(["-m", "--max"]),
        device: pargs
            .opt_value_from_str("-d")?
            .or_else(|| pargs.opt_value_from_str("--device").unwrap_or(None)),
        action: pargs.opt_free_from_str()?,
    };

    let _ = pargs.finish();

    Ok(args)
}

#[inline]
fn calculate_percentage(total: i32, value: i32) -> i32 {
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
    -l, --list       Lists all available devices.
    -m, --max        Prints the maximum brightness value.
    -M, --monitor    Monitors the brightness for changes.

OPTIONS:
    -d, --device Target a specific device.

ACTIONS:
    nothing          Returns the current brightness value.
    =value           Sets the current brightness value.
    +value           Increases the current brightness value.
    -value           Decreases the current brightness value.
    =percentage%     Sets the current brightness value as a percentage.
    +percentage%     Increases the current brightness value by a percentage.
    -percentage%     Decreases the current brightness value by a percentage.

EXAMPLES:
    l1ght +50        Increases the current brightness value by 50.
    l1ght -5%        decreases the current brightness value by 5%",
        VERSION
    );
}
