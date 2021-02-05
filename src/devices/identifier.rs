use libusb::DeviceDescriptor;

use crate::devices::protocol::Protocol;

const SUPPORTED_DEVICES: &[Identifier] = &[Identifier {
    vendor_id: 0x045e,
    product_id: 0x02ea,
    protocol: Protocol {
        class: 0xff,
        subclass: 71,
        protocol: 208,
    },
}];

pub struct Identifier {
    pub vendor_id: u16,
    pub product_id: u16,
    pub(super) protocol: Protocol,
}

impl Identifier {
    pub(super) fn find_supported<'a>(
        descriptor: &'a DeviceDescriptor,
    ) -> Option<&'static Identifier> {
        let vendor_id = descriptor.vendor_id();
        let product_id = descriptor.product_id();

        SUPPORTED_DEVICES
            .iter()
            .find(|i| i.vendor_id == vendor_id && i.product_id == product_id)
    }
}
