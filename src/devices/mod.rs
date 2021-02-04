//! Contains device definitions and their parsing/creation functionality

mod controller;
mod device_info;
mod endpoints;
mod identifier;
mod protocol;

pub use crate::devices::controller::Controller;
use crate::devices::device_info::DeviceInfo;
use crate::devices::identifier::Identifier;
use libusb::{Context, Result};

/// Returns all controllers on USB that are supported by the package
pub fn find_supported_controllers(context: &Context) -> Result<Vec<Controller>> {
    let devices = context.devices()?;

    let mut controllers = Vec::new();
    for device in devices.iter() {
        let descriptor = device.device_descriptor()?;
        let identifier = match Identifier::find_supported(&descriptor) {
            Some(value) => value,
            None => continue,
        };
        let handle = device.open()?;
        let device_info = DeviceInfo::read(&handle, &descriptor)?;
        let endpoints = endpoints::find(&device, &descriptor, &identifier.protocol)?;

        controllers.push(Controller::new(identifier, device_info, handle, endpoints))
    }

    Ok(controllers)
}
