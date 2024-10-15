use godot::prelude::*;
use godot::classes::Node;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct CharacterBodyController {
    base: Base<Node>
}

#[godot_api]
impl INode for CharacterBodyController {

    fn init(base: Base<Node>) -> Self {
        Self { 
            base
        }
    }
}

#[godot_api]
impl CharacterBodyController {

    #[func(virtual)]
    fn get_horizontal_movement(&mut self) -> Vector3 {
        Vector3::ZERO
    }

    #[func(virtual)]
    fn action_pressed(&mut self, _action_name: StringName) -> bool {
        false
    }

    #[func(virtual)]
    fn action_just_pressed(&mut self, _action_name: StringName) -> bool {
        false
    }

    #[func(virtual)]
    fn action_released(&mut self, _action_name: StringName) -> bool {
        false
    }

}