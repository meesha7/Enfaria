extends Control

func toggle_inventory():
    var inv = get_node("Main")
    inv.visible = !inv.visible
    
func hide_inventory():
    var inv = get_node("Main")
    inv.visible = false

func get_drag_data(position):
    var collided = get_world_2d().direct_space_state.intersect_point(position, 1, [], 4)
    if len(collided) != 1:
        return null
    var slot = collided[0].collider.get_parent()
    if !slot.occupied:
        return null
    return slot.get_children()[1]

func can_drop_data(position, data):
    if !("item_name" in data):
        return false
    var collided = get_world_2d().direct_space_state.intersect_point(position, 1, [], 4)
    if len(collided) != 1:
        return false
    var slot = collided[0].collider.get_parent()
    if slot.occupied:
        return false
    return true

func drop_data(position, data):
    var new_slot = get_world_2d().direct_space_state.intersect_point(position, 1, [], 4)[0].collider.get_parent()
    var previous_slot = data.get_parent()
    previous_slot.occupied = false
    previous_slot.remove_child(data)
    new_slot.add_child(data)
    new_slot.occupied = true
    
    var from = previous_slot.name.right(4)
    var to = new_slot.name.right(4)
    get_node("/root/connection").generate_packet(Dictionary({"MoveItem":[int(from), int(to)]}))
