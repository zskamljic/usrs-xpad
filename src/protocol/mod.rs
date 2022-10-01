mod keys;
mod parsing;
mod protocols;

use crate::devices::Controller;
use crate::protocol::Command::Terminate;
pub use keys::*;
use libusb::Error;

pub struct Protocol {
    init_packets: Vec<InitPacket>,
}

struct InitPacket {
    order: Option<usize>,
    after_packet: Option<Vec<u8>>,
    content: Vec<u8>,
}

impl Protocol {
    pub fn for_ids(vendor_id: u16, product_id: u16) -> Option<Protocol> {
        match (vendor_id, product_id) {
            (0x045e, 0x02ea) => Some(protocols::create_xbox_one_s()),
            (vendor, product) => {
                println!(
                    "Unknown vendor_id and product_id pair: {:04x}:{:04x}",
                    vendor, product
                );
                None
            }
        }
    }

    pub fn init(&self, controller: &mut Controller) -> Option<Command> {
        let mut index = 0;
        while index < self.init_packets.len() {
            match controller.read() {
                Ok(data) => {
                    let packets = self.find_response_packet(data, index);
                    for packet in packets {
                        if controller.write(packet).is_err() {
                            return Some(Terminate);
                        }
                        index += 1;
                    }
                }
                Err(Error::Timeout) => return None,
                Err(error) => {
                    println!("Unable to read: {}", error);
                    return Some(Command::Terminate);
                }
            };
        }
        None
    }

    pub fn read_command(&self, controller: &mut Controller) -> Option<Command> {
        match controller.read() {
            Ok(data) => parsing::parse_packet(&data),
            Err(Error::Timeout) => None,
            Err(_) => Some(Terminate),
        }
    }

    fn find_response_packet(&self, data: Vec<u8>, index: usize) -> Vec<&Vec<u8>> {
        self.init_packets
            .iter()
            .filter(|p| {
                if let Some(after) = &p.after_packet {
                    after.iter().zip(&data).filter(|&(a, b)| a == b).count() == after.len()
                } else if let Some(order) = p.order {
                    index == order
                } else {
                    false
                }
            })
            .map(|p| &p.content)
            .collect()
    }
}
