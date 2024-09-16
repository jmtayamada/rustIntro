use godot::prelude::*;
use godot::classes::{ISprite2D, Sprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player2D {
    #[export]
    speed: f32,
    // angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player2D {
    fn init(base: Base<Sprite2D>) -> Self {        
        Self {
            speed: 400.0,
            // angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity: Vector2 = Vector2::from_tuple((0.0, 0.0));

        let input = Input::singleton();

        velocity += Vector2::from_array(
            [
                input.get_axis("left".into(), "right".into()), 
                input.get_axis("forward".into(), "back".into())
            ]
        );

        velocity = velocity.normalized() * self.speed;

        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        // let position = Vector2::new(
        //     position.x.clamp(0.0, self.screen_size.x),
        //     position.y.clamp(0.0, self.screen_size.y),
        // );
        self.base_mut().set_global_position(position);

        if input.is_action_just_pressed("pause".into()) {
            self.increase_speed(50 as f32);
        }
    }
}

#[godot_api]
impl Player2D {
    #[func]
    fn increase_speed(&mut self, amount: f32) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}
