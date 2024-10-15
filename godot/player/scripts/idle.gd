extends State

@export
var move_state: State
#@export
#var fall_state: State
#@export
#var jump_state: State

func _enter() -> void:
	enter_default()
	
func _exit() -> void:
	pass

func _process_input(event: InputEvent) -> State:
	if Input.is_action_just_pressed('back') or Input.is_action_just_pressed("forward") or Input.is_action_just_pressed("left") or Input.is_action_just_pressed("right"):
		return move_state
	return null
	
func _process_physics(_delta: float) -> State:
	parent_node().velocity.y -= parent_node().fall_gravity

	parent_node().move_and_slide()
		
	#if !parent_node.is_on_floor():
		#return fall_state
	return null
