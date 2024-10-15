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
	
	var velocity = controller._get_horizontal_movement() * parent_node.speed
	velocity.y -= parent_node.fall_gravity
	parent_node.velocity = velocity
	
	#if parent_node.velocity.y < 0:
		#return fall_state

	parent_node.move_and_slide()
	
	if parent_node.is_on_floor():
		if parent_node.velocity.is_zero_approx():
			return idle_state
	
	return null
