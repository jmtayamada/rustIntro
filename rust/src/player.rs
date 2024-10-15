use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::{AnimationPlayer, CharacterBody3D, ICharacterBody3D, InputEvent};

use crate::character_controller::CharacterBodyController;
use crate::state_machine::StateMachine;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    animations: Option<Gd<AnimationPlayer>>,
    state_machine: Option<Gd<StateMachine>>,
    character_controller: Option<Gd<CharacterBodyController>>,

    // jump variables
    jump_speed: f32,
    #[export]
    jump_height: f32,
    #[export]
    jump_peak_time: f32,
    #[export]
    jump_fall_time: f32,
    #[export]
    jump_distance: f32,
    #[var]
    jump_gravity: f32,
    #[var]
    fall_gravity: f32,
    #[var]
    jump_velocity: f32,

    // movement variables
    #[export]
    speed: f32,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {        
        Self {
            animations: None,
            state_machine: None,
            character_controller: None,

            // jump variables
            jump_speed: 0.0,
            jump_height: 6.0,
            jump_peak_time: 0.3,
            jump_fall_time: 0.3,
            jump_distance: 18.0,
            jump_gravity: 0.0,
            fall_gravity: 0.0,
            jump_velocity: 0.0,

            // movement variables
            speed: 6.0,

            base
        }
    }

    fn ready(&mut self) {

        let _node = self.base_mut().get_node_as("AnimationPlayer");
        self.animations = Some(_node);

        let _node = self.base_mut().get_node_as("StateMachine");
        self.state_machine = Some(_node);

        let _node = self.base_mut().get_node_as("CharacterBodyController");
        self.character_controller = Some(_node);

        let _self = self.base().clone();
        match self.state_machine.as_mut() {
            Some(state_machine) => {state_machine.bind_mut().initialize(_self.cast(), self.animations.clone(), self.character_controller.clone())},
            None => {godot_error!("state machine not found")},
        }

        self.calculate_movement_params();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        match self.state_machine.as_mut() {
            Some(state_machine) => state_machine.bind_mut().process_input(event),
            None => {godot_error!("state machine not found")},
        }
    }

    fn physics_process(&mut self, delta: f64) {
        match self.state_machine.as_mut() {
            Some(state_machine) => state_machine.bind_mut().process_physics(delta),
            None => {godot_error!("state machine not found")},
        }
    }

    fn process(&mut self, delta: f64) {
        match self.state_machine.as_mut() {
            Some(state_machine) => state_machine.bind_mut().process_frame(delta),
            None => {godot_error!("state machine not found")},
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    pub fn get_animation_player(&mut self) -> Option<Gd<AnimationPlayer>> {
        if let Some(animation_player) = self.animations.as_mut() {
            return Some(animation_player.clone())
        } else {
            return None
        }
    }

    #[func] // calculate basic movement parameters
    fn calculate_movement_params(&mut self) {
        self.jump_gravity = (2.0 * self.jump_height)/(self.jump_peak_time.powf(2.0));
        self.fall_gravity = (2.0 * self.jump_height)/(self.jump_fall_time.powf(2.0));
        self.jump_velocity = self.jump_gravity * self.jump_peak_time;
        self.jump_speed = self.jump_distance/(self.jump_peak_time + self.jump_fall_time);
    }
}
