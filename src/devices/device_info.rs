use std::time::Duration;

use libusb::{DeviceDescriptor, DeviceHandle, Error, Result};

#[derive(Debug)]
pub struct DeviceInfo {
    pub manufacturer: String,
    pub name: String,
    pub serial: String,
}

impl DeviceInfo {
    pub(super) fn read(handle: &DeviceHandle, descriptor: &DeviceDescriptor) -> Result<DeviceInfo> {
        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout)?;
        let language = match languages.first() {
            Some(value) => *value,
            None => return Err(Error::NotFound),
        };

        let manufacturer = handle.read_manufacturer_string(language, descriptor, timeout)?;
        let name = handle.read_product_string(language, descriptor, timeout)?;
        let serial = handle.read_serial_number_string(language, descriptor, timeout)?;

        Ok(DeviceInfo {
            manufacturer,
            name,
            serial,
        })
    }
}
