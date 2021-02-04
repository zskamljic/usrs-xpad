#![warn(missing_docs)]
//! Crate to (hopefully) provide a working userspace driver
//! for XBOX controllers, like xpad

use crossbeam::thread;
use crossbeam::thread::ScopedJoinHandle;
use libusb::{Context, Error, Result};

use devices::Controller;

mod devices;

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
    let mut packets = vec![
        vec![0x04, 0x20, 0x01, 0x00],
        vec![
            0x01, 0x20, 0x07, 0x09, 0x00, 0x1e, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    ];
    let mut next_packet = None;
    loop {
        match controller.read() {
            Ok(data) => {
                if packets.is_empty() {
                    continue;
                }
                if data.len() > 2 && data[0] == 0x02 && data[1] == 0x20 {
                    next_packet = Some(packets.remove(0));
                }
            }
            Err(Error::Timeout) => {
                if packets.is_empty() {
                    continue;
                }
                next_packet = Some(packets.remove(0));
            }
            Err(error) => {
                println!("Unable to read: {}", error);
                break;
            }
        }
        if let Some(mut packet) = next_packet {
            if let Err(error) = controller.write(&mut packet) {
                println!("Unable to write: {}", error);
                break;
            }
            next_packet = None;
        }
    }
    Ok(())
}
