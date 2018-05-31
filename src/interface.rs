use std::path::PathBuf;
use std::fs;
use std::clone::Clone;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct Interface {
    path: PathBuf,
    max: i32,
}

impl Interface {
    fn new(p: PathBuf) -> Self {
        let mut max_path: PathBuf = p.clone();
        max_path.push("max_brightness");
        let mut max_file = File::open(max_path).expect("oh no max");
        let mut max_str = String::new();
        max_file.read_to_string(&mut max_str);
        // remove \n
        max_str.pop();
        let max: i32 = max_str.parse().unwrap();
        Interface { path: p, max }
    }

    pub fn exists(&self) -> bool {
        self.path.as_path().exists()
    }

    pub fn get_max(&self) -> &i32 {
        &self.max
    }

    pub fn brightness(&self) -> i32 {
        let mut birght_path = PathBuf::new();
        birght_path.clone_from(&self.path);
        birght_path.push("brightness");
        let mut bright_file = File::open(birght_path).expect("oh no brightness");
        let mut bright_str = String::new();
        bright_file.read_to_string(&mut bright_str);
        // remove \n
        bright_str.pop();
        let brightness: i32 = bright_str.parse().unwrap();
        brightness
    }

    pub fn set_brightness(&self, data: i32) {
        let mut path = PathBuf::new();
        path.clone_from(&self.path);
        path.push("brightness");

        let mut opened_fie = OpenOptions::new()
            .write(true)
            .open(path)
            .expect("oh no set");
        let mut mystr = String::new();
        opened_fie.read_to_string(&mut mystr);
        println!("{}", mystr);
        opened_fie
            .write_all(&data.to_string().as_bytes())
            .expect("Unable to write data");
    }

    pub fn increase_brightness(&self, value: i32) {
        let mut new_brightness = &self.brightness() + value;

        if new_brightness > self.max {
            new_brightness = self.max
        }

        &self.set_brightness(new_brightness);
    }

    pub fn decrease_brightness(&self, value: i32) {
        let mut new_brightness = &self.brightness() - value;

        if new_brightness < 0 {
            new_brightness = 0
        }

        &self.set_brightness(new_brightness);
    }
}

pub fn get_interfaces() -> Vec<Interface> {
    let mut interfaces: Vec<Interface> = Vec::new();
    let interfaces_path = fs::read_dir("/sys/class/backlight/").unwrap();

    for dir in interfaces_path {
        let actual_dir = dir.unwrap();
        interfaces.push(Interface::new(actual_dir.path()));
    }

    interfaces
}

pub fn get_interface(name: &String) -> Option<Interface> {
    let mut path = PathBuf::from("/sys/class/backlight/".to_string());
    path.push(name);

    if path.as_path().exists() {
            return Some(Interface::new(path));
    }
    None
}
