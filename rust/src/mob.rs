use godot::{
    classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[func]
    fn on_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Mob { base }
    }

    fn ready(&mut self) {
        godot_print!("hello world from enemy");
        let mut animated_sprites = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mob_types = animated_sprites
            .get_sprite_frames()
            .unwrap()
            .get_animation_names();

        let animation_names: Array<GString> = mob_types.as_slice().into();
        let rand_animation = animation_names.pick_random().unwrap();

        animated_sprites.set_animation(rand_animation.arg());
        animated_sprites.play();
    }
}
