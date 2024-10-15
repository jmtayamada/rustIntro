extends CharacterBodyController

func _get_horizontal_movement() -> Vector3:
	var x_movement = Input.get_axis('left', 'right')
	var z_movement = Input.get_axis('forward', 'back')
	return Vector3(x_movement, 0, z_movement).normalized()

func _action_pressed(action_name):
	return Input.is_action_pressed(action_name)
	
func _action_just_pressed(action_name):
	return Input.is_action_just_pressed(action_name)

func _action_released(action_name):
	return Input.is_action_just_released(action_name)
