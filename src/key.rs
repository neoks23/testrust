use gdnative::api::Area2D;
use gdnative::api::CollisionShape2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Key {
    float_var: f32,
    float_speed: f32,
    float_boundaries: f32,
    dir: bool,
}

#[gdnative::methods]
impl Key {
    fn new(_owner: &Area2D) -> Self {
        Key {
            float_var: 0.0,
            float_speed: 0.0,
            float_boundaries: 4.0,
            dir: true,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Area2D) {
        godot_print!("KEYYY.");
        self.float_speed = self.float_boundaries / 20.0;
    }

    #[export]
    fn _process(&mut self, _owner: &Area2D, delta: f32) {
        let sprite = unsafe { _owner.get_node_as::<CollisionShape2D>("Sprite").unwrap() };

        if self.float_var >= self.float_boundaries {
            self.float_var = self.float_boundaries;
            self.dir = false;
        }

        if self.float_var <= -self.float_boundaries {
            self.float_var = -self.float_boundaries;
            self.dir = true;
        }

        if self.dir {
            self.float_var = self.float_var + self.float_speed;
        } else {
            self.float_var = self.float_var - self.float_speed;
        }

        sprite.set_position(Vector2::new(0.0, self.float_var));
    }
    #[export]
    fn _on_Key_body_entered(&mut self, _owner: &Area2D, body: Ref<Node>) {
        godot_print!("something touchy touchy");
        let body = unsafe { body.assume_safe() };

        if body.name() == GodotString::from_str("Bliep") {
            godot_print!("it was bliep!");
            _owner.queue_free();
        }
    }
}
