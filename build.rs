extern crate cc;

fn main() {
    cc::Build::new().file("csrc/uinput.c").compile("constants")
}
