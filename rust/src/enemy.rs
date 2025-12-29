use godot::{
    classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
struct Enemy {
    base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for Enemy {
    fn init(base: Base<RigidBody2D>) -> Self {
        Enemy { base }
    }

    fn ready(&mut self) {
        let mut animated_sprites = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mob_types = animated_sprites
            .get_sprite_frames()
            .unwrap()
            .get_animation_names();

        let animation_names: Array<GString> = mob_types.as_slice().iter().cloned().collect();
        let rand_animation = animation_names.pick_random().unwrap();

        animated_sprites.set_animation(rand_animation.arg());
        animated_sprites.play();
    }
}
