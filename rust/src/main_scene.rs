use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    base: Base<Node>,

    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,
    score: u32,
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
