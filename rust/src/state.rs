use godot::prelude::*;
use godot::classes::{InputEvent, Node};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node)] // no_init
pub struct State {
    #[export]
    animation_name: GString,
    #[var]
    parent_node_path: NodePath,
    base: Base<Node>
}

#[godot_api]
impl INode for State {

    fn init(base: Base<Node>) -> Self {
        Self { 
            animation_name: "".into(), 
            parent_node_path: "".into(), 
            base
        }
    }
}

#[godot_api]
impl State {
    #[func]
    pub fn parent_node(&mut self) -> Option<Gd<Player>> {
        let string = self.parent_node_path.clone();
        Some(self.base_mut().get_node_as::<Player>(string))
    }

    #[func(virtual)]
    pub fn set_parent_node_self(&mut self, path_to_parent: NodePath) {
        self.parent_node_path = path_to_parent;
    }

    #[func(virtual)]
    pub fn enter(&mut self) {
        if let Some(parent) = self.parent_node().as_mut() {
            parent.bind_mut().play_animation(self.animation_name.clone());
        } else {
            godot_error!("No parent found")
        }
    }

    #[func]
    pub fn enter_default(&mut self) {
        if let Some(parent) = self.parent_node().as_mut() {
            parent.bind_mut().play_animation(self.animation_name.clone());
        } else {
            godot_error!("No parent found")
        }
    }

    #[func(virtual)]
    pub fn exit(&mut self) {

    }

    #[func(virtual)]
    pub fn process_input(&mut self, _event: Gd<InputEvent>) -> Option<Gd<State>> {
        None
    }

    #[func(virtual)]
    pub fn process_frame(&mut self, _delta: f64) -> Option<Gd<State>> {
        None
    }

    #[func(virtual)]
    pub fn process_physics(&mut self, _delta: f64) -> Option<Gd<State>> {
        None
    }
}