use godot::prelude::*;
use godot::classes::{AnimationPlayer, CharacterBody3D, InputEvent, Node};

use crate::character_controller::CharacterBodyController;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct State {
    #[export]
    animation_name: GString,
    #[var]
    parent_node: Option<Gd<CharacterBody3D>>,
    #[var]
    animation_player: Option<Gd<AnimationPlayer>>,
    #[var]
    controller: Option<Gd<CharacterBodyController>>,
    base: Base<Node>
}

#[godot_api]
impl INode for State {

    fn init(base: Base<Node>) -> Self {
        Self { 
            animation_name: "".into(), 
            parent_node: None, 
            animation_player: None,
            controller: None,
            base
        }
    }
}

#[godot_api]
impl State {

    #[func(virtual)]
    pub fn set_parameters(&mut self, input_node: Option<Gd<CharacterBody3D>>, animation_player_node: Option<Gd<AnimationPlayer>>, character_controller_node: Option<Gd<CharacterBodyController>>) {
        self.parent_node = input_node;
        self.animation_player = animation_player_node;
        self.controller = character_controller_node;
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
    pub fn process_input(&mut self, _event: Gd<InputEvent>) {
    }

    #[func(virtual)]
    pub fn process_frame(&mut self, _delta: f64) {
        if let Some(animation_player) = self.animation_player.as_mut() {
            animation_player.play();
        }
    }

    #[func(virtual)]
    pub fn process_physics(&mut self, _delta: f64) {
    }

    #[signal]
    fn change_state(state: Option<Gd<State>>);
}