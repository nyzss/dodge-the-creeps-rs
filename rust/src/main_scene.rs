use godot::{
    classes::{Marker2D, Timer},
    prelude::*,
};

use crate::player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    base: Base<Node>,

    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,
    score: u32,
}

#[godot_api]
impl Main {
    #[func]
    fn game_over(&mut self) {
        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");

        score_timer.stop();
        mob_timer.stop();
    }

    #[func]
    fn new_game(&mut self) {
        self.score = 0;

        let mut start_timer = self.base().get_node_as::<Timer>("StartTimer");
        let start_position = self.base().get_node_as::<Marker2D>("StartPosition");

        let mut player = self.base().get_node_as::<player::Player>("Player");

        player.bind_mut().start(start_position.get_position());
        start_timer.start();
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            score: 0,
            mob_scene: OnEditor::default(),
        }
    }
}
