use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    #[export]
    speed: i32,
    screen_size: Vector2,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        return Player {
            speed: 400,
            screen_size: Vector2 { x: 720., y: 400. },
            base,
        };
    }

    fn physics_process(&mut self, delta: f32) {
        self.base_mut().rotate(std::f32::consts::PI * delta as f32);
    }
}
