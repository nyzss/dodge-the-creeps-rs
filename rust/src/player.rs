use godot::classes::{AnimatedSprite2D, Area2D, IArea2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    #[export]
    speed: i32,
    screen_size: Vector2,
    animated_sprite: OnReady<Gd<AnimatedSprite2D>>,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Player {
            speed: 400,
            screen_size: Vector2::ZERO,
            base,
            animated_sprite: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        let viewport_rect = self.base().get_viewport_rect();

        self.screen_size = Vector2::new(viewport_rect.size.x, viewport_rect.size.y);

        let animated_sprite = self.base().get_node_as("AnimatedSprite2D");

        self.animated_sprite.init(animated_sprite);
    }

    fn physics_process(&mut self, delta: f32) {
        let input = Input::singleton();

        let mut direction = Vector2::ZERO;
        if input.is_action_pressed("move_up") {
            direction += Vector2::UP;
        }
        if input.is_action_pressed("move_down") {
            direction += Vector2::DOWN;
        }
        if input.is_action_pressed("move_right") {
            direction += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left") {
            direction += Vector2::LEFT;
        }

        let velocity = direction * self.speed as f32 * delta as f32;

        if velocity.length() > 0. {
            let animation: &str;

            if velocity.x != 0. {
                animation = "walk";

                self.animated_sprite.set_flip_v(false);
                self.animated_sprite.set_flip_h(velocity.x < 0.);
            } else {
                animation = "up";

                self.animated_sprite.set_flip_v(velocity.y > 0.);
            }

            self.animated_sprite.play_ex().name(animation).done();
        } else {
            self.animated_sprite.stop();
        }

        let position = self.base().get_global_position();

        let new_position = (position + velocity).clamp(Vector2::ZERO, self.screen_size);

        self.base_mut().set_global_position(new_position);
    }
}
