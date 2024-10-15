use godot::prelude::*;
use godot::classes::{AnimationPlayer, CharacterBody3D, InputEvent, Node};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct State {
    #[export]
    animation_name: GString,
    #[var]
    parent_node: Option<Gd<CharacterBody3D>>,
    #[var]
    animation_player: Option<Gd<AnimationPlayer>>,
    base: Base<Node>
}

#[godot_api]
impl INode for State {

    fn init(base: Base<Node>) -> Self {
        Self { 
            animation_name: "".into(), 
            parent_node: None, 
            animation_player: None,
            base
        }
    }
}

#[godot_api]
impl State {

    #[func(virtual)]
    pub fn set_parameters(&mut self, input_node: Option<Gd<CharacterBody3D>>, animation_player_node: Option<Gd<AnimationPlayer>>) {
        self.parent_node = input_node;
        self.animation_player = animation_player_node;
    }

    #[func(virtual)]
    pub fn enter(&mut self) {
        if let Some(animation_player) = self.animation_player.as_mut() {
            animation_player.play_ex().name(self.animation_name.clone().into()).done();
        } else {
            godot_error!("No animation player")
        }
    }

    #[func]
    pub fn enter_default(&mut self) {
        if let Some(animation_player) = self.animation_player.as_mut() {
            animation_player.play_ex().name(self.animation_name.clone().into()).done();
        } else {
            godot_error!("No animation player")
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