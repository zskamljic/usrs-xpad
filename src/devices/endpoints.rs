use std::collections::HashMap;

use libusb::{Device, DeviceDescriptor, Direction, InterfaceDescriptor, Result, TransferType};

use crate::devices::protocol::Protocol;

pub(super) type Endpoints = HashMap<TransferType, Endpoint>;

#[derive(Debug)]
pub(super) struct Endpoint {
    pub(super) interface: u8,
    pub(super) config: u8,
    pub(super) setting: u8,
    pub(super) input: u8,
    pub(super) output: u8,
}

pub(super) fn find(
    device: &Device,
    descriptor: &DeviceDescriptor,
    protocol: &Protocol,
) -> Result<Endpoints> {
    let mut endpoints = Endpoints::new();

    for config in 0..descriptor.num_configurations() {
        let config_descriptor = device.config_descriptor(config)?;
        let config = config_descriptor.number();

        let interfaces: Vec<InterfaceDescriptor> = config_descriptor
            .interfaces()
            .flat_map(|i| i.descriptors())
            .filter(|i| protocol.supports(i))
            .collect();

        for interface in interfaces.iter() {
            let interface_number = interface.interface_number();
            let setting_number = interface.setting_number();
            let mut input = None;
            let mut output = None;
            let mut transfer_type = None;

            for endpoint_descriptor in interface.endpoint_descriptors() {
                match endpoint_descriptor.direction() {
                    Direction::In => input = Some(endpoint_descriptor.address()),
                    Direction::Out => output = Some(endpoint_descriptor.address()),
                }
                transfer_type = Some(endpoint_descriptor.transfer_type());
            }
            if let (Some(transfer_type), Some(input), Some(output)) = (transfer_type, input, output)
            {
                endpoints.insert(
                    transfer_type,
                    Endpoint {
                        interface: interface_number,
                        setting: setting_number,
                        config,
                        input,
                        output,
                    },
                );
            }
        }
    }

    Ok(endpoints)
}
