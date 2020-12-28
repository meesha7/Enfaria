extends Control

func get_drag_data(position):
	var collided = get_world_2d().direct_space_state.intersect_point(position, 1, [], 4)
	if len(collided) != 1:
		return null
	var slot = collided[0].collider.get_parent()
	if !slot.occupied:
		return null
	return slot.get_node_or_null("Item")

func can_drop_data(position, data):
	if !(data is Item):
		return false
	var collided = get_world_2d().direct_space_state.intersect_point(position, 1, [], 4)
	if len(collided) != 1:
		return false
	var slot = collided[0].collider.get_parent()
	if slot.occupied:
		return false
	return true

func drop_data(position, data):
	var collided = get_world_2d().direct_space_state.intersect_point(position, 1, [], 4)[0].collider.get_parent()
	data.get_parent().occupied = false
	data.get_parent().remove_child(data)
	collided.add_child(data)
	collided.occupied = true
