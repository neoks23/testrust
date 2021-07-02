use gdnative::api::StaticBody2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Lock;

#[gdnative::methods]
impl Lock {
    fn new(_owner: &StaticBody2D) -> Self {
        Lock
    }

    #[export]
    fn _on_lock_body_entered(&mut self, _owner: &StaticBody2D, body: Ref<Node>) {
        let body = unsafe { body.assume_safe() };
        if body.name() == GodotString::from_str("Bliep") {
            if body.get(GodotString::from_str("has_key")).to_bool() {
                body.set(GodotString::from_str("has_key"), false);
                _owner.queue_free();
            }
        }
    }
}
