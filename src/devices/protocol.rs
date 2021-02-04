use libusb::InterfaceDescriptor;

pub(super) struct Protocol {
    pub(super) class: u8,
    pub(super) subclass: u8,
    pub(super) protocol: u8,
}

impl Protocol {
    pub(super) fn supports(&self, descriptor: &InterfaceDescriptor) -> bool {
        self.class == descriptor.class_code()
            && self.subclass == descriptor.sub_class_code()
            && self.protocol == descriptor.protocol_code()
    }
}
