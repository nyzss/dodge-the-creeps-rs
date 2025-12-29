use std::f32::consts::PI as PI32;
use std::f64::consts::PI as PI64;

use godot::{
    classes::{Marker2D, PathFollow2D, Timer},
    global::{randf, randf_range},
    prelude::*,
};

use crate::{mob, player};

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    base: Base<Node>,

    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,
    score: u32,

    mob_timer: OnReady<Gd<Timer>>,
    score_timer: OnReady<Gd<Timer>>,
    start_timer: OnReady<Gd<Timer>>,
}

#[godot_api]
impl Main {
    #[func]
    fn game_over(&mut self) {
        self.score_timer.stop();
        self.mob_timer.stop();
    }

    #[func]
    fn new_game(&mut self) {
        let start_position = self.base().get_node_as::<Marker2D>("StartPosition");
        let mut player = self.base().get_node_as::<player::Player>("Player");

        self.score = 0;
        player.bind_mut().start(start_position.get_position());
        self.start_timer.start();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        self.mob_timer.start();
        self.score_timer.start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob = self.mob_scene.instantiate_as::<mob::Mob>();
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        mob_spawn_location.set_progress_ratio(randf() as f32);

        mob.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI32 / 2.0;

        direction += randf_range(-PI64 / 4.0, PI64 / 4.0) as f32;
        mob.set_rotation(direction);

        let velocity = Vector2::new(randf_range(150.0, 250.0) as f32, 0.0);
        mob.set_linear_velocity(velocity.rotated(direction));

        self.base_mut().add_child(&mob);
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            score: 0,
            mob_scene: OnEditor::default(),

            mob_timer: OnReady::manual(),
            score_timer: OnReady::manual(),
            start_timer: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        let main = self.to_gd();

        self.mob_timer
            .init(self.base().get_node_as::<Timer>("MobTimer"));
        self.score_timer
            .init(self.base().get_node_as::<Timer>("ScoreTimer"));
        self.start_timer
            .init(self.base().get_node_as::<Timer>("StartTimer"));

        self.mob_timer
            .signals()
            .timeout()
            .connect_other(&main, Self::on_mob_timer_timeout);

        self.new_game();
    }
}
