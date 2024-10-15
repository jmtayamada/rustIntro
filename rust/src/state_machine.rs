use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::{InputEvent, Node, CharacterBody3D};

use crate::state::State;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct StateMachine {
    #[export]
    starting_state: Option<Gd<State>>,
    current_state: Option<Gd<State>>,
    base: Base<Node>
}

#[godot_api]
impl INode for StateMachine {

    fn init(base: Base<Node>) -> Self {
        Self { 
            starting_state: None, 
            current_state: None, 
            base
        }
    }
}

#[godot_api]
impl StateMachine {

    #[func]
    pub fn initialize(&mut self, parent: Gd<CharacterBody3D>) {
        for node in self.base_mut().get_children().iter_shared() {
            let path = node.get_path_to(parent.clone());
            {
                let mut child = node.cast::<State>();
                child.bind_mut().set_parent_node_self(path);
            }
        }

        // self.change_state(self.starting_state);

        if let Some(starting_state) = self.starting_state.as_ref() {
            self.change_state(starting_state.clone());
        } else {
            godot_error!("No starting state")
        }

    }

    #[func]
    pub fn change_state(&mut self, new_state: Gd<State>) {

        if let Some(current_state) = self.current_state.as_mut() {
            current_state.call_deferred("_exit".into(), &[]);
        }

        self.current_state = Some(new_state);

        if let Some(current_state) = self.current_state.as_mut() {
            current_state.call_deferred("_enter".into(), &[]);
        }

    }

    #[func]
    pub fn process_physics(&mut self, delta: f64) {

        let mut new_state: Option<Gd<State>> = None;
        if let Some(state) = self.current_state.as_mut() {
            new_state = state.call_deferred("_process_physics".into(), &[delta.to_variant()]).to();
        }

        if let Some(state) = new_state {
            self.change_state(state);
        }
    }

    #[func]
    pub fn process_input(&mut self, event: Gd<InputEvent>) {

        let mut new_state: Option<Gd<State>> = None;
        if let Some(state) = self.current_state.as_mut() {
            new_state = state.bind_mut().process_input(event);
        }

        if let Some(state) = new_state {
            self.change_state(state);
        }

    }

    #[func]
    pub fn process_frame(&mut self, delta: f64) {

        let mut new_state: Option<Gd<State>> = None;
        if let Some(state) = self.current_state.as_mut() {
            new_state = state.bind_mut().process_frame(delta);
        }
        if let Some(state) = new_state {
            self.change_state(state);
        }
    }
}