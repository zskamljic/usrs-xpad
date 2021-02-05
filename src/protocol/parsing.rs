use crate::protocol::Command::Xbox;
use crate::protocol::{AlphaKeys, BackKeys, Command, DirectionKeys, MetaKeys, Stick};

pub(super) fn parse_packet(data: &[u8]) -> Option<Command> {
    // XBOX key packet
    if data.len() == 6 && data[0] == 0x07 {
        return Some(Xbox(data[4] != 0));
    }
    // Not a valid key packet
    if data.len() != 18 {
        println!("Refused to parse packet with len: {}", data.len());
        return None;
    }
    match data[0] {
        0x20 => Some(parse_button_data(data)),
        value => {
            println!("Unknown packet type: {}", value);
            None
        }
    }
}

pub(super) fn parse_button_data(data: &[u8]) -> Command {
    Command::Keys {
        meta: MetaKeys {
            select: data[4] & 0b0000_0100 != 0,
            back: data[4] & 0b0000_1000 != 0,
        },
        alpha: AlphaKeys {
            a: data[4] & 0b0001_0000 != 0,
            b: data[4] & 0b010_0000 != 0,
            x: data[4] & 0b0100_0000 != 0,
            y: data[4] & 0b1000_0000 != 0,
        },
        direction: DirectionKeys {
            up: data[5] & 0b0000_0001 != 0,
            down: data[5] & 0b0000_0010 != 0,
            left: data[5] & 0b0000_0100 != 0,
            right: data[5] & 0b0000_1000 != 0,
        },
        back: BackKeys {
            lb: data[5] & 0b0001_0000 != 0,
            rb: data[5] & 0b0010_0000 != 0,
            lt: ((data[7] as u16) << 8) | data[6] as u16,
            rt: ((data[9] as u16) << 8) | data[8] as u16,
        },
        left_stick: Stick {
            x: ((data[11] as i16) << 8) | data[10] as i16,
            y: ((data[13] as i16) << 8) | data[12] as i16,
            clicked: data[5] & 0b0100_0000 != 0,
        },
        right_stick: Stick {
            x: ((data[15] as i16) << 8) | data[14] as i16,
            y: ((data[17] as i16) << 8) | data[16] as i16,
            clicked: data[5] & 0b1000_0000 != 0,
        },
    }
}
