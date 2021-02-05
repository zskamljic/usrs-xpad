#![warn(missing_docs)]
//! Crate to (hopefully) provide a working userspace driver
//! for XBOX controllers, like xpad

use crossbeam::thread;
use crossbeam::thread::ScopedJoinHandle;
use libusb::{Context, Result};

use crate::protocol::{Command, Protocol};
use crate::uinput::UInputHandle;
use devices::Controller;

mod devices;
mod mapping;
mod protocol;
mod uinput;

fn main() -> Result<()> {
    let context = Context::new()?;
    let controllers = devices::find_supported_controllers(&context)?;

    thread::scope(move |scope| {
        let handles: Vec<ScopedJoinHandle<Result<()>>> = controllers
            .into_iter()
            .map(|c| scope.spawn(move |_| controller_loop(c)))
            .collect();
        for handle in handles {
            match handle.join() {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    })
    .unwrap();
    loop {}
}

fn controller_loop(mut controller: Controller) -> Result<()> {
    let vendor = controller.identifier.vendor_id;
    let product = controller.identifier.product_id;
    let uinput = match UInputHandle::new(
        "Microsoft X-Box One S pad",
        // &format!(
        //     "{} {}",
        //     controller.device_info.manufacturer, controller.device_info.name
        // ),
        vendor,
        product,
    ) {
        Ok(value) => value,
        Err(error) => {
            println!("Unable to init uinput: {}", error);
            return Err(libusb::Error::NoDevice);
        }
    };
    let protocol = match Protocol::for_ids(vendor, product) {
        Some(value) => value,
        None => return Ok(()),
    };
    controller.prepare()?;
    protocol.init(&mut controller);
    loop {
        if let Some(packet) = protocol.read_command(&mut controller) {
            match packet {
                Command::Terminate => break,
                Command::Keys { .. } | Command::Xbox(_) => mapping::apply_keys(&uinput, packet),
            }
        }
    }
    Ok(())
}
