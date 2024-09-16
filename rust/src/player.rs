use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::{AnimationPlayer, CharacterBody3D, ICharacterBody3D, Timer};
use crate::attack::Attack;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    speed: f32,
    #[export]
    jump_height: f32,
    #[export]
    jump_peak_time: f32,
    #[export]
    jump_fall_time: f32,
    #[export]
    jump_distance: f32,
    jump_gravity: f32,
    fall_gravity: f32,
    jump_velocity: f32,

    #[export]
    air_dash_distance: f32,
    #[export]
    air_dash_time: f64,
    #[export]
    ground_dash_distance: f32,
    #[export]
    ground_dash_time: f64,
    #[export]
    dash_cooldown: f64,
    #[export]
    max_dash_amount: i8,
    dash_available: i8,
    // is_dashing: bool,
    air_dash_speed: f32,
    ground_dash_speed: f32,

    #[export]
    max_hp: i16,
    current_hp: i16,
    max_shield: i16,
    current_shield: i16,

    #[export]
    num_basic_ground_attacks: i8,
    current_basic_ground_attack: i8,
    #[export]
    num_basic_air_attacks: i8,
    current_basic_air_attack: i8,

    last_facing_direction: Vector3,

    current_state: String,  // idle, moving, ground_dashing, air_dashing, jumping, falling, ground_attacking, air_attacking

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {        
        Self {
            speed: 0.0,
            jump_height: 6.0,
            jump_peak_time: 0.3,
            jump_fall_time: 0.3,
            jump_distance: 18.0,
            jump_gravity: 0.0,
            fall_gravity: 0.0,
            jump_velocity: 0.0,

            air_dash_distance: 15.0,
            air_dash_time: 0.25,
            ground_dash_distance: 15.0,
            ground_dash_time: 0.25,
            max_dash_amount: 5,
            dash_available: 0,
            // is_dashing: false,
            dash_cooldown: 0.5,
            air_dash_speed: 0.0,
            ground_dash_speed: 0.0,

            max_hp: 50,
            current_hp: 0,
            max_shield: 50,
            current_shield: 0, 

            num_basic_ground_attacks: 3,
            current_basic_ground_attack: 0,
            num_basic_air_attacks: 2,
            current_basic_air_attack: 0,

            current_state: "idle".into(),

            last_facing_direction: Vector3::from_tuple((1.0, 0.0, 1.0)),

            base,
        }
    }

    fn ready(&mut self) {
        self.calculate_movement_params();

        let dash_cooldown = self.dash_cooldown;
        self.base_mut().get_node_as::<Timer>("DashRefresh").set_wait_time(dash_cooldown);

        let air_dash_time = self.air_dash_time;
        self.base_mut().get_node_as::<Timer>("AirDashTime").set_wait_time(air_dash_time);

        let ground_dash_time = self.ground_dash_time;
        self.base_mut().get_node_as::<Timer>("GroundDashTime").set_wait_time(ground_dash_time);

        self.dash_available = self.max_dash_amount;
        self.current_hp = self.max_hp;
        self.current_shield = self.max_shield;

        self.num_basic_ground_attacks -= 1;
        self.num_basic_air_attacks -= 1;
    }

    fn physics_process(&mut self, _delta: f64) {

    }
}

#[godot_api]
impl Player {
    #[func] // calculate basic movement parameters
    fn calculate_movement_params(&mut self) {
        self.jump_gravity = (2.0 * self.jump_height)/(self.jump_peak_time.powf(2.0));
        self.fall_gravity = (2.0 * self.jump_height)/(self.jump_fall_time.powf(2.0));
        self.jump_velocity = self.jump_gravity * self.jump_peak_time;
        self.speed = self.jump_distance/(self.jump_peak_time + self.jump_fall_time);
        self.air_dash_speed = self.air_dash_distance/(self.air_dash_time as f32);
        self.ground_dash_speed = self.ground_dash_distance/(self.ground_dash_time as f32);
    }


    // code for dealing with dash
    #[func]
    pub fn increase_dash_counter(&mut self) {
        if self.dash_available < self.max_dash_amount {
            self.dash_available += 1;
        }
    }

    #[func]
    fn disable_dash_state(&mut self) {
        if self.base_mut().is_on_floor() {
            self.modify_state_bypass_dash("idle".into());
        } else {
            self.modify_state_bypass_dash("falling".into());
        }
    }

    // process inputs, call appropriate functions
    #[func]
    pub fn process_inputs(&mut self, input: Dictionary, delta: f64) {

        let mut velocity: Vector3 = Vector3::ZERO;

        velocity = self.movement(
            velocity, 
            input.at("x_axis").to(), 
            input.at("y_axis").to(),
        );

        velocity = self.jump(velocity, input.at("jump").to(), delta);

        velocity = self.dash(velocity, input.at("dash").to());

        velocity = self.basic_attack(input.at("attack").to(), self.base().is_on_floor(), velocity);

        {
            let y_holder = velocity.y;
            velocity.y = 0.0;
            if velocity != Vector3::from_tuple((0.0, 0.0, 0.0)) {
                self.last_facing_direction = velocity;
                self.last_facing_direction.y = 0.0;
            }
            velocity.y = y_holder;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        self.animations();
    }

    #[func]
    fn animations(&mut self) {
        let mut animation_player = self.base_mut().get_node_as::<AnimationPlayer>("AnimationPlayer");

        if self.current_state == "idle" {
            animation_player.set_speed_scale(1.0);
            animation_player.play_ex().name("float".into()).done();
        } else if self.current_state == "moving" {
            animation_player.set_speed_scale(2.0);
            animation_player.play_ex().name("float".into()).done();
        } else if self.current_state == "jumping" {
            animation_player.set_speed_scale(1.0);
            animation_player.play_ex().name("jump".into()).done();
        } else if self.current_state == "ground_dashing" {
            animation_player.set_speed_scale(1.0);
            animation_player.play_ex().name("dash".into()).done();
        } else if self.current_state == "falling" {
            animation_player.set_speed_scale(1.0);
            animation_player.play_ex().name("fall".into()).done();
        } else if self.current_state == "air_dashing" {
            animation_player.set_speed_scale(4.0);
            animation_player.play_ex().name("air_dash".into()).done();
        } else if self.current_state == "ground_attacking" {
            let mut animation: String = "ground_attack_".to_string();
            let num: String = self.current_basic_ground_attack.to_string();
            animation.push_str(&num);
            animation_player.set_speed_scale(1.0);
            animation_player.play_ex().name(animation.into()).done();
        } else if self.current_state == "air_attacking" {
            let mut animation: String = "air_attack_".to_string();
            let num: String = self.current_basic_air_attack.to_string();
            animation.push_str(&num);
            animation_player.set_speed_scale(1.0);
            animation_player.play_ex().name(animation.into()).done();
        }
        
    }

    // movement functions
    #[func]
    fn dash(&mut self, movement_vector: Vector3, dash: bool) -> Vector3 {

        let mut vector: Vector3 = movement_vector;

        if dash && self.dash_available > 0 {
            self.dash_available -= 1;

            if self.base().is_on_floor() {
                self.modify_state_bypass_dash("ground_dashing".to_string());
                self.base_mut().get_node_as::<Timer>("GroundDashTime").start();
            } else {
                self.modify_state_bypass_dash("air_dashing".to_string());
                self.base_mut().get_node_as::<Timer>("AirDashTime").start();
            }
        }

        if self.current_state.eq("ground_dashing") {
            let y = vector.y;
            vector = self.last_facing_direction.normalized();
            vector.y = y;
            vector.x *= self.ground_dash_speed;
            vector.z *= self.ground_dash_speed;
        } else if self.current_state.eq("air_dashing") {
            let y = vector.y;
            vector = self.last_facing_direction.normalized();
            vector.y = y;
            vector.x *= self.air_dash_speed;
            vector.z *= self.air_dash_speed;
        }

        if self.dash_available < self.max_dash_amount && self.base().get_node_as::<Timer>("DashRefresh").is_stopped() {
            self.base_mut().get_node_as::<Timer>("DashRefresh").start();
        }

        vector
    }

    #[func]
    fn movement(&mut self, movement_vector: Vector3, x_axis: f32, y_axis: f32) -> Vector3 {
        let mut velocity: Vector3 = movement_vector;

        velocity += Vector3::from_array(
            [
                x_axis, 
                0.0,
                y_axis,
            ]
        );

        if !velocity.is_zero_approx() {
            velocity = velocity.normalized();
            velocity.x *= self.speed;
            velocity.z *= self.speed;
            self.modify_state("moving".into());
        } else {
            self.modify_state("idle".into());
        }

        velocity
    }

    #[func]
    fn jump(&mut self, movement_vector: Vector3, jump: bool, delta: f64) -> Vector3 {
        let mut velocity = movement_vector;
        velocity.y = self.base().get_velocity().y;
        if jump && CharacterBody3D::is_on_floor(&self.base()) {
            velocity.y = self.jump_velocity;
            self.modify_state_bypass_dash("jumping".into());
        }

        if !self.base().is_on_floor() {
            if self.base().get_velocity().y > 0.0 {
                velocity.y -= self.jump_gravity * delta as f32;
                self.modify_state("jumping".into());
            } else {
                velocity.y -= self.fall_gravity * delta as f32;
                self.modify_state("falling".into());
            }
        }

        velocity
    }

    // combat functions
    #[func]
    fn basic_attack(&mut self, attack: bool, on_floor: bool, velocity: Vector3) -> Vector3 {
        let mut velocity = velocity;
        if attack && !self.current_state.eq("ground_attacking") && !self.current_state.eq("air_attacking") {
            if on_floor {
                self.modify_state_bypass_dash("ground_attacking".into());

                let mut path: String = "Pivot/GroundAttacks/Attack".to_string();
                let attack_string: String = self.current_basic_ground_attack.to_string();
                path.push_str(&attack_string);
                let mut _node = self.base_mut().get_node_as::<Attack>(path);
                let mut attack_node = _node.bind_mut();
                attack_node.begin_attack();
            } else {
                self.modify_state_bypass_dash("air_attacking".into());

                let mut path: String = "Pivot/AirAttacks/Attack".to_string();
                let attack_string: String = self.current_basic_air_attack.to_string();
                path.push_str(&attack_string);
                let mut _node = self.base_mut().get_node_as::<Attack>(path);
                let mut attack_node = _node.bind_mut();
                attack_node.begin_attack();
            }
        }

        if self.current_state.eq("ground_attacking") || self.current_state.eq("air_attacking") {
            velocity.x = 0.0;
            velocity.z = 0.0;
        }

        velocity
    }

    #[func]
    pub fn end_attack(&mut self) {
        if self.base_mut().is_on_floor() {
            self.modify_state_bypass_dash("idle".into());
            if self.current_basic_ground_attack < self.num_basic_ground_attacks {
                self.current_basic_ground_attack += 1;
            } else {
                self.current_basic_ground_attack = 0;
            }
        } else {
            self.modify_state_bypass_dash("falling".into());
            if self.current_basic_air_attack < self.num_basic_air_attacks {
                self.current_basic_air_attack += 1;
            } else {
                self.current_basic_air_attack = 0;
            }
        }
    }

    #[func]
    pub fn end_chain(&mut self, ground_attack: bool) {
        if ground_attack {
            self.current_basic_ground_attack = 0;
        } else {
            self.current_basic_air_attack = 0;
        }
    }

    // state handler
    #[func]
    pub fn modify_state(&mut self, state: String) {
        if !self.current_state.eq("air_dashing") && !self.current_state.eq("ground_dashing") && !self.current_state.eq("ground_attacking") && !self.current_state.eq("air_attacking"){
            self.current_state = state;
        }
    }

    #[func]
    pub fn modify_state_bypass_dash(&mut self, state: String) {
        self.current_state = state;
    }

    // hp/shield functions
    #[func]
    fn modify_hp(&mut self, change: i16) {
        self.current_hp += change;
    }

    #[func]
    fn modify_shield(&mut self, change: i16) {
        self.current_shield += change;
    }

    #[func]
    pub fn rotate_pivot(&mut self) {
        if self.last_facing_direction.cross(Vector3::from_tuple((0.0, 1.0, 0.0))) != Vector3::from_tuple((0.0, 0.0, 0.0)) {
            let mut pivot_node = self.base_mut().get_node_as::<Node3D>("Pivot");
            pivot_node.set_basis(Basis::new_looking_at(self.last_facing_direction, Vector3::from_tuple((0.0, 1.0, 0.0)), true));
        }
    }
}
