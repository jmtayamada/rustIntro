use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::{Area3D, IArea3D, Timer};

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct Attack {
    #[export]
    damage_multiplyer: f32,
    #[export]
    attack_length: f64,
    #[export]
    chargeable: bool,
    #[export]
    ground_attack: bool,
    #[export]
    chain_length: f64,

    base: Base<Area3D>,
}

#[godot_api]
impl IArea3D for Attack {
    fn init(base: Base<Area3D>) -> Self {
        Self {
            damage_multiplyer: 1.0,
            attack_length: 0.5,
            chargeable: false,
            ground_attack: true,
            chain_length: 0.25,

            base,
        }
    }

    fn ready(&mut self) {
        let mut timer = Timer::new_alloc();
        self.base_mut().add_child(&timer);
        timer.set_wait_time(self.attack_length);
        timer.set_one_shot(true);
        timer.set_name("AttackLength".into());
        let callable: Callable = self.base_mut().callable("end_attack");
        timer.connect("timeout".into(), callable);

        let mut timer2 = Timer::new_alloc();
        self.base_mut().add_child(&timer2);
        timer2.set_wait_time(self.chain_length);
        timer2.set_one_shot(true);
        timer2.set_name("AttackChain".into());
        let callable: Callable = self.base_mut().callable("end_chain");
        timer2.connect("timeout".into(), callable);
    }
}

#[godot_api]
impl Attack {
    #[func]
    pub fn begin_attack(&mut self) {
        let mut chain_timer = self.base_mut().get_node_as::<Timer>("AttackChain");

        chain_timer.stop();

        if chain_timer.get_time_left() == 0.0 {
            godot_print!("timer reset");
        }

        self.base_mut().set_monitoring(true);
        let mut timer = self.base_mut().get_node_as::<Timer>("AttackLength");
        timer.start();
    }

    #[func]
    pub fn end_attack(&mut self) {
        self.base_mut().set_monitoring(false);
        self.base_mut().emit_signal("attack_ended".into(), &[]);

        let mut chain_timer = self.base_mut().get_node_as::<Timer>("AttackChain");
        chain_timer.start();
    }

    #[func]
    pub fn end_chain(&mut self) {
        godot_print!("end chain");
        let boolean: bool = self.ground_attack;
        self.base_mut().emit_signal("chain_ended".into(), &[boolean.to_variant()]);
    }

    #[signal]
    fn attack_ended();

    #[signal]
    fn chain_ended(ground_attack: bool);
}