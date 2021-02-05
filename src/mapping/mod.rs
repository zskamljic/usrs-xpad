use crate::protocol::{AlphaKeys, BackKeys, Command, DirectionKeys, MetaKeys, Stick};
use crate::uinput::{Key, UInputHandle};

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
            send_meta_keys(input, meta);
            send_alpha_keys(input, alpha);
            send_direction_keys(input, direction);
            send_back_keys(input, back);
            send_left_stick(input, left_stick);
            send_right_stick(input, right_stick);
            input.commit_keys();
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

fn send_direction_keys(input: &UInputHandle, keys: DirectionKeys) {}

fn send_back_keys(input: &UInputHandle, keys: BackKeys) {
    input.set_key_pressed(Key::LB, keys.lb);
    input.set_key_pressed(Key::RB, keys.rb);
}

fn send_left_stick(input: &UInputHandle, stick: Stick) {}

fn send_right_stick(input: &UInputHandle, stick: Stick) {}
