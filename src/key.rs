use gdnative::api::Area2D;
use gdnative::api::CollisionShape2D;
use gdnative::prelude::*;

use crate::bliep;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Key {
    float_var: f32,
    frame: f32,
    #[property]
    float_speed: f32,
    #[property]
    float_step: f32,
    #[property]
    float_boundaries: f32,
    #[property]
    xydir: bool,
    idir: bool,
    dir: bool,
}

#[gdnative::methods]
impl Key {
    fn new(_owner: &Area2D) -> Self {
        Key {
            float_var: 0.0,
            frame: 1.01,
            float_speed: 700.0,
            float_step: 0.5,
            float_boundaries: 6.0,
            xydir: false,
            idir: true,
            dir: true,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Area2D) {
        self.idir = true;
    }

    #[export]
    fn _process(&mut self, _owner: &Area2D, delta: f32) {
        self.floating_animation(_owner);
    }

    #[export]
    fn floating_animation(&mut self, _owner: &Area2D) {
        let sprite = unsafe { _owner.get_node_as::<CollisionShape2D>("Sprite").unwrap() };

        self.float_var = self.ease_out_quad(
            _owner,
            self.frame,
            self.frame * 100.0,
            0.0,
            self.float_boundaries,
            self.float_speed,
        );

        if self.dir {
            if !self.xydir {
                sprite.set_position(Vector2::new(0.0, self.float_var));
            } else {
                sprite.set_position(Vector2::new(self.float_var, 0.0));
            }
        } else {
            if !self.xydir {
                sprite.set_position(Vector2::new(0.0, -self.float_var));
            } else {
                sprite.set_position(Vector2::new(-self.float_var, 0.0));
            }
        }
        if self.idir {
            self.frame = self.frame + self.float_step;
        } else {
            self.frame = self.frame - self.float_step;
        }

        if self.float_var >= self.float_boundaries - 1.0 {
            self.idir = false;
        }
        if self.float_var <= 1.0 {
            if !self.idir {
                self.dir = !self.dir;
            }
            self.idir = true;
        }
    }

    //x = percent completed
    //t = time elapsed
    //b = start point
    //c = end point
    //d = total ms
    #[export]
    fn ease_in_quad(&mut self, _owner: &Area2D, x: f32, t: f32, b: f32, c: f32, d: f32) -> f32 {
        let t = t / d;
        c * t * t + b
    }

    //x = percent completed
    //t = time elapsed
    //b = start point
    //c = end point
    //d = total ms
    #[export]
    fn ease_out_quad(&mut self, _owner: &Area2D, x: f32, t: f32, b: f32, c: f32, d: f32) -> f32 {
        let t = t / d;
        -c * t * (t - 2.0) + b
    }

    //x = percent completed
    //t = time elapsed
    //b = start point
    //c = end point
    //d = total ms
    #[export]
    fn ease_in_out_quad(&mut self, _owner: &Area2D, x: f32, t: f32, b: f32, c: f32, d: f32) -> f32 {
        let t = t / d;
        if t / 2.0 < 1.0 {
            return c / 2.0 * t * t + b;
        }

        let t = t - 1.0;

        -c / 2.0 * ((t) * (t - 2.0) - 1.0) + b
    }

    #[export]
    fn _on_key_body_entered(&mut self, _owner: &Area2D, body: Ref<Node>) {
        let body = unsafe { body.assume_safe() };

        if body.name() == GodotString::from_str("Bliep") {
            body.set(GodotString::from_str("has_key"), true);
            _owner.queue_free();
        }
    }
}
