use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

#[gdnative::methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("hello, world.");
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f32) {
        let sprite = unsafe { _owner.get_node_as::<Sprite>("Sprite").unwrap() };
        //sprite.set_global_scale(sprite.scale() + Vector2::new(0.05, 0.02));
        let position = sprite.global_position() + Vector2::new(1.0, 0.0);
        sprite.set_global_position(position);
        sprite.set_global_rotation_degrees(sprite.global_rotation_degrees() + 10.0);
    }

    #[export]
    fn _on_btn_quit_button_up(&self, _owner: &Node) {
        let tree = _owner.get_tree().expect("Couldn't find scene tree");
        let tree = unsafe { tree.assume_safe() };
        tree.change_scene("bla.tscn")
            .expect("scene could not be loaded");
    }
}
