use godot::{
    builtin::Vector2,
    classes::{ISprite2D, Sprite2D},
    obj::{Base, WithBaseField},
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base = Sprite2D)]
struct Player {
    base: Base<Sprite2D>,

    speed: f32,
    angular_speed: f32,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            speed: 400.,
            angular_speed: std::f32::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = self.angular_speed * delta as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f32) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased", &[]);
    }

    #[signal]
    fn speed_increased();
}
