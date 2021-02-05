use std::fmt;
use std::fmt::{Debug, Formatter};

use libusb::{DeviceHandle, Result, TransferType};

use crate::devices::device_info::DeviceInfo;
use crate::devices::endpoints::Endpoints;
use crate::devices::identifier::Identifier;
use std::time::Duration;

pub struct Controller<'a> {
    pub identifier: &'static Identifier,
    pub device_info: DeviceInfo,
    handle: DeviceHandle<'a>,
    endpoints: Endpoints,
}

impl<'a> Controller<'a> {
    pub(super) fn new(
        identifier: &'static Identifier,
        device_info: DeviceInfo,
        handle: DeviceHandle<'a>,
        endpoints: Endpoints,
    ) -> Controller<'a> {
        Controller {
            identifier,
            device_info,
            handle,
            endpoints,
        }
    }

    pub fn prepare(&mut self) -> Result<()> {
        let endpoint = &self.endpoints[&TransferType::Interrupt];
        self.handle.reset()?;
        if self.handle.kernel_driver_active(endpoint.interface)? {
            println!("Detaching kernel driver");
            self.handle.detach_kernel_driver(endpoint.interface)?;
        }
        self.handle.set_active_configuration(endpoint.config)?;
        self.handle.claim_interface(endpoint.interface)?;
        self.handle
            .set_alternate_setting(endpoint.interface, endpoint.setting)?;
        Ok(())
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        let endpoint = &self.endpoints[&TransferType::Interrupt];
        let mut buffer = [0u8; 64];
        let read =
            self.handle
                .read_interrupt(endpoint.input, &mut buffer, Duration::from_secs(1))?;
        Ok(buffer[0..read].to_vec())
    }

    pub fn write(&self, data: &[u8]) -> Result<()> {
        let endpoint = &self.endpoints[&TransferType::Interrupt];
        self.handle
            .write_interrupt(endpoint.output, data, Duration::from_secs(0))?;
        Ok(())
    }
}

impl Debug for Controller<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Controller")
            .field(
                "identifier",
                &format!(
                    "{:04x}:{:04x}",
                    self.identifier.vendor_id, self.identifier.product_id
                ),
            )
            .field("device_info", &self.device_info)
            .field("endpoints", &self.endpoints)
            .finish()
    }
}
