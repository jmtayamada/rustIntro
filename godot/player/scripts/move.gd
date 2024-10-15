extends State

@export
var idle_state: State
@export
var fall_state: State
#@export
#var jump_state: State

func _enter() -> void:
	enter_default()
	
func _exit() -> void:
	pass
	
func _process_frame(_delta: float):
	pass
	
func _process_input(_event):
	pass
	
func _process_physics(delta: float):
	var velocity = controller._get_horizontal_movement() * parent_node.speed
	parent_node.velocity.x = velocity.x
	parent_node.velocity.z = velocity.z
	parent_node.velocity.y -= parent_node.fall_gravity*delta

	parent_node.move_and_slide()
	
	if parent_node.is_on_floor():
		if velocity.is_zero_approx():
			change_state.emit(idle_state)
	else:
		if parent_node.velocity.y < 0:
			change_state.emit(fall_state)
