extends State

@export
var idle_state: State
#@export
#var fall_state: State
#@export
#var jump_state: State

func _enter() -> void:
	enter_default()
	
func _exit() -> void:
	pass
	
func _process_physics(delta: float) -> State:
	parent_node().velocity.y -= parent_node().fall_gravity
	
	#if parent_node.velocity.y < 0:
		#return fall_state
	
	var x_movement = Input.get_axis('left', 'right') * parent_node().speed
	var z_movement = Input.get_axis('forward', 'back') * parent_node().speed

	parent_node().velocity.x = x_movement
	parent_node().velocity.z = z_movement
	parent_node().move_and_slide()
	
	if parent_node().is_on_floor():
		if parent_node().velocity.is_zero_approx():
			return idle_state
	
	return null
