use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    base: Base<Area2D>,

    #[export]
    speed: i32,
    screen_size: Vector2,
    animated_sprite: OnReady<Gd<AnimatedSprite2D>>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func]
    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        self.base_mut().hide();

        self.signals().hit().emit();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision_shape.set_deferred("disabled", &true.to_variant());
    }

    #[func]
    fn start(&mut self, pos: Vector2) {
        self.base_mut().set_position(pos);

        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision_shape.set_disabled(false);
    }
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
        // self.base_mut().hide();

        let viewport_rect = self.base().get_viewport_rect();

        self.screen_size = Vector2::new(viewport_rect.size.x, viewport_rect.size.y);

        let animated_sprite = self.base().get_node_as("AnimatedSprite2D");

        self.animated_sprite.init(animated_sprite);

        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);
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
