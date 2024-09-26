/// Sys-FS based backlight control
///
/// Stable ABI: https://www.kernel.org/doc/Documentation/ABI/stable/sysfs-class-backlight
use std::io;
use std::fs;
use std::path::PathBuf;

const SYS_CLASS_BACKLIGHT: &str = "/sys/class/backlight";
const MAX_BRIGTHNESS: &str = "max_brightness";
const BRIGHTNESS: &str = "brightness";
const ACTUAL_BRIGHTNESS: &str = "actual_brightness"; 

#[derive(Debug, Clone)]
pub struct DeviceId(pub String);

/// Lists all available backlight devices.
pub fn list_devices() -> Result<Vec<DeviceId>, io::Error> {
    let mut devices = Vec::new();

    for entry in fs::read_dir(SYS_CLASS_BACKLIGHT)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        devices.push(DeviceId(file_name));
    }

    Ok(devices)
}

/// Opens a backlight device by its id.
/// 
/// Returns an error if the device does not exist.
/// 
/// The device does not need to be closed.
pub fn open_device(device_id: DeviceId) -> Result<Backlight, io::Error> {
    Backlight::open(device_id)
}

/// Represents a backlight device.
/// 
/// Use `open_device` to get an instance.
pub struct Backlight {
    /// Folder name in /sys/class/backlight
    id: String,
    /// cached value of the max_brightness file, 
    /// it should not change during operation.
    max_brightness: i32,
}

fn backlight_path(id: &str) -> PathBuf {
    PathBuf::from(SYS_CLASS_BACKLIGHT).join(id)
}

fn read_value_from_path(path: &PathBuf, file: &str) -> Result<String, io::Error> {
    fs::read_to_string(path.join(file))
}

fn parse_numeric_value(value: &str) -> Result<i32, io::Error> {
    value
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn read_numeric_value_from_path(path: &PathBuf, file: &str) -> Result<i32, io::Error> {
    read_value_from_path(path, file).and_then(|v| parse_numeric_value(&v))
}

impl Backlight {
    pub(self) fn open(device_id: DeviceId) -> Result<Self, io::Error> {
        let id = device_id.0;
        let path = PathBuf::from(SYS_CLASS_BACKLIGHT).join(&id);
        let device_exists = fs::exists(&&path)?;

        if !device_exists {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Device {} does not exist.", id),
            ))
        }

        let max_brightness = read_numeric_value_from_path(&path, MAX_BRIGTHNESS)?;

        Ok(Backlight {
            id,
            max_brightness,
        })
    }

    /// Returns the maximum brightness value reported by the driver.
    pub fn get_max_brightness(&self) -> i32 {
        self.max_brightness
    }

    /// Gets the current brightness value reported by the driver.
    /// 
    /// This value may be different from the actual brightness value.
    /// Use `get_actual_brightness` to query the hardware. 
    pub fn get_brightness(&self) -> Result<i32, io::Error> {
        read_numeric_value_from_path(&backlight_path(&self.id), BRIGHTNESS)
    }

    /// Sets the brightness value. 
    pub fn set_brightness(&self, requested_value: i32) -> Result<(), io::Error> {
        let value = requested_value.min(self.max_brightness);

        fs::write(
            backlight_path(&self.id).join(BRIGHTNESS),
            value.to_string(),
        )
    }

    /// Gets the actual brightness value by querying the hardware.
    /// 
    /// use `get_brightness` for cached value reported by the driver. 
    pub fn get_actual_brightness(&self) -> Result<i32, io::Error> {
        read_numeric_value_from_path(&backlight_path(&self.id), ACTUAL_BRIGHTNESS)
    }

    /// Increases the brightness by the given value.
    pub fn increase_brightness(&self, value: i32) -> Result<(), io::Error> {
        let current = self.get_actual_brightness()?;
        self.set_brightness(current + value)
    }

    /// Decreases the brightness by the given value.
    pub fn decrease_brightness(&self, value: i32) -> Result<(), io::Error> {
        let current = self.get_actual_brightness()?;
        self.set_brightness(current - value)
    }
}
