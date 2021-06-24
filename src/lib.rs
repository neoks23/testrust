use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct HelloWorld;

#[gdnative::methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.");
        let sprite = unsafe { _owner.get_node_as::<Sprite>("Sprite").unwrap() };
        sprite.set_global_rotation_degrees(45.0);
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f32) {
        let sprite = unsafe { _owner.get_node_as::<Sprite>("Sprite").unwrap() };
        let position = sprite.global_position() + Vector2::new(1.0, 0.0);
        sprite.set_global_position(position);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);
