extends State

@export
var move_state: State
@export
var idle_state: State
#@export
#var jump_state: State

func _enter() -> void:
	enter_default()
	
func _exit() -> void:
	pass

func _process_input(_event: InputEvent):
	pass
	
func _process_frame(_delta: float):
	pass
	
func _process_physics(delta: float):
	parent_node.velocity.y -= parent_node.fall_gravity*delta

	parent_node.move_and_slide()
		
	if parent_node.is_on_floor():
		if controller._get_horizontal_movement().is_equal_approx(Vector3.ZERO):
			change_state.emit(idle_state)
		else:
			change_state.emit(move_state)
