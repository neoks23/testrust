use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bliep {
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub gravity: f32,
    pub jump_height: f32,
    jumping: bool,
    velocity: Vector2,
}

#[gdnative::methods]
impl Bliep {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bliep {
            speed: 300.0,
            max_speed: 500.0,
            acceleration: 0.0,
            gravity: 10.0,
            jump_height: 400.0,
            jumping: false,
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
        self.jumping = false;
    }

    #[export]
    fn _process(&mut self, _owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();

        if Input::is_action_pressed(&input, GodotString::from_str("left")) {
            self.velocity.x = -self.acceleration;
        } else if Input::is_action_pressed(&input, GodotString::from_str("right")) {
            self.velocity.x = self.acceleration;
        } else {
            if self.velocity.x < 0.0 {
                self.velocity.x = -self.acceleration;
            }
            if self.velocity.x > 0.0 {
                self.velocity.x = self.acceleration;
            }
            self.acceleration = self.acceleration - 20.0;
            if self.acceleration < 0.0 {
                self.acceleration = 0.0;
            }
            if self.acceleration == 0.0 {
                self.velocity.x = self.acceleration;
            }
        }

        if self.acceleration < self.speed {
            self.acceleration = self.acceleration + 5.0;
        }
        if Input::is_action_pressed(&input, GodotString::from_str("run")) {
            if Input::is_action_pressed(&input, GodotString::from_str("left"))
                || Input::is_action_pressed(&input, GodotString::from_str("right"))
                    && self.acceleration < self.max_speed
            {
                self.acceleration = self.acceleration + 2.0;
                if self.acceleration > self.max_speed {
                    self.acceleration = self.max_speed;
                }
            }
        } else if !Input::is_action_pressed(&input, GodotString::from_str("run")) {
            if self.acceleration > self.speed {
                self.acceleration = self.acceleration - 10.0;
            }
        }

        if Input::is_action_pressed(&input, GodotString::from_str("up"))
            && _owner.is_on_floor()
            && !self.jumping
        {
            self.jumping = true;
            self.velocity.y = self.velocity.y + -self.jump_height;
        }

        if self.jumping && self.velocity.y == 0.0 {
            self.jumping = false;
        }
        self.velocity.y = self.velocity.y + self.gravity;

        self.velocity = _owner.move_and_slide(
            self.velocity * delta * 60.0,
            Vector2::new(0.0, 1.0),
            false,
            64,
            45.0,
            true,
        );
    }
}
