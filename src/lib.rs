use gdnative::prelude::*;

mod bliep;
mod helloworld;
mod key;

fn init(handle: InitHandle) {
    handle.add_class::<helloworld::HelloWorld>();
    handle.add_class::<bliep::Bliep>();
    handle.add_class::<key::Key>();
}

godot_init!(init);
