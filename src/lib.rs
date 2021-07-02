use gdnative::prelude::*;

mod bliep;
mod helloworld;
mod key;
mod lock;

fn init(handle: InitHandle) {
    handle.add_class::<helloworld::HelloWorld>();
    handle.add_class::<bliep::Bliep>();
    handle.add_class::<key::Key>();
    handle.add_class::<lock::Lock>();
}

godot_init!(init);
