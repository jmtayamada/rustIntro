use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::{Node, INode};
use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Manager {
    #[export]
    camera_position: Vector3,
    #[export]
    camera_rotation: Vector3,
    // #[export]
    // character: Gd<Player>,

    base: Base<Node>,
}

#[godot_api]
impl INode for Manager {
    fn init(base: Base<Node>) -> Self {        
        Self {
            camera_position: Vector3{x: 0.0, y: 11.0, z: 10.0},
            camera_rotation: Vector3{x: -51.0, y: 0.0, z: 0.0},

            base,
        }
    }

    fn ready(&mut self) {
        let mut camera = self.base().get_node_as::<Camera3D>("../MainCamera");
        camera.set_position(self.camera_position);
        camera.set_rotation(self.camera_rotation);
        if camera.get_parent().is_some() {
            self.base().get_node_as::<Node>("..").call_deferred("remove_child".into(), &[camera.to_variant()]);
        }
        self.base_mut().get_node_as::<Player>("Player").call_deferred("add_child".into(), &[camera.to_variant()]);
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let mut binding = self.base().get_node_as::<Player>("Player");
        let mut player_node = binding.bind_mut();
        player_node.process_inputs(
            dict! {
                "x_axis": input.get_axis("left".into(), "right".into()),
                "y_axis": input.get_axis("forward".into(), "back".into()),
                "jump": input.is_action_just_pressed("jump".into()),
                "dash": input.is_action_just_pressed("dash".into()),
                "pause": input.is_action_just_pressed("jump".into()),
                "attack": input.is_action_just_pressed("attack".into()),
            }, 
            delta
        );
        player_node.rotate_pivot();
    }
}

#[godot_api]
impl Manager {
    #[func]
    fn get_player(&mut self) {

    }
}
