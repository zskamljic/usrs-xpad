use crate::protocol::{AlphaKeys, BackKeys, Command, DirectionKeys, MetaKeys, Stick};
use crate::uinput::{Key, Side, UInputHandle};

pub fn apply_keys(input: &UInputHandle, command: Command) {
    match command {
        Command::Terminate => return,
        Command::Xbox(pressed) => send_xbox_key(input, pressed),
        Command::Keys {
            meta,
            alpha,
            direction,
            back,
            left_stick,
            right_stick,
        } => {
            let left_trigger = back.lt;
            let right_trigger = back.rt;
            send_meta_keys(input, meta);
            send_alpha_keys(input, alpha);
            send_direction_keys(input, direction);
            send_back_keys(input, back);
            send_left_stick(input, left_stick, left_trigger);
            send_right_stick(input, right_stick, right_trigger);
        }
    }
}

fn send_xbox_key(input: &UInputHandle, pressed: bool) {
    input.set_key_pressed(Key::Xbox, pressed);
}

fn send_meta_keys(input: &UInputHandle, keys: MetaKeys) {
    input.set_key_pressed(Key::Back, keys.back);
    input.set_key_pressed(Key::Select, keys.select);
}

fn send_alpha_keys(input: &UInputHandle, keys: AlphaKeys) {
    input.set_key_pressed(Key::A, keys.a);
    input.set_key_pressed(Key::B, keys.b);
    input.set_key_pressed(Key::X, keys.x);
    input.set_key_pressed(Key::Y, keys.y);
}

fn send_direction_keys(input: &UInputHandle, keys: DirectionKeys) {
    input.set_key_pressed(Key::Up, keys.up);
    input.set_key_pressed(Key::Down, keys.down);
    input.set_key_pressed(Key::Left, keys.left);
    input.set_key_pressed(Key::Right, keys.right);
}

fn send_back_keys(input: &UInputHandle, keys: BackKeys) {
    input.set_key_pressed(Key::LB, keys.lb);
    input.set_key_pressed(Key::RB, keys.rb);
}

fn send_left_stick(input: &UInputHandle, stick: Stick, trigger: u16) {
    input.update_axis(Side::Left, stick.x, stick.y, trigger);
}

fn send_right_stick(input: &UInputHandle, stick: Stick, trigger: u16) {
    input.update_axis(Side::Right, stick.x, stick.y, trigger);
}
