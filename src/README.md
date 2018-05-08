# l1ght

This is a small cli, which lets you control the brightness of your laptop screen.

## Motivation
I mainly wrote this because xbacklight, stopped working on my fresh Debian install for some reason, sadly I could not fix it. So I was looking for a alternative and learned some stuff about, how linux handles that kind of stuff.

I am well aware that there are better alternatives, but I kind of wanted to write something myself, mainly as an exercise and also for the lulz.

## Usage
this comes directly from the "-h/--help" argument.

```
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
```

### License
This code is licensed under GPL3.

Please refer to the "LICENSE" file for details.


### Packaging
You can build packages, for your platform yourself.
Currently the Packaging is done for Debian and Arch Linux.
Alternatively you could also install it via `cargo install`, but this only makes the program available for your user.
For building a package of your choice call `make arch` or `make debian`.
