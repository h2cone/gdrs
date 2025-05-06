use godot::prelude::*;

use crate::player;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Main {
    player: OnReady<Gd<player::Player>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            player: OnReady::from_node("Player"),
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Main ready");
        let main = self.to_gd();
        self.player
            .signals()
            .speed_increased()
            .connect_obj(&main, Self::on_player_speed_increased);
    }
}

#[godot_api]
impl Main {
    fn on_player_speed_increased(&mut self) {
        godot_print!("Player speed increased");
    }
}
