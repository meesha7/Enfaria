extends Control

var dragging = null

func _input(event):
    if !(event is InputEventMouseButton):
        return

    if event.button_index != BUTTON_LEFT:
        return

    if event.is_pressed():
        var collided = get_world_2d().direct_space_state.intersect_point(event.position, 1, [], 4)
        if len(collided) != 1:
            return

        var slot = collided[0].collider.get_parent()
        if !slot.occupied:
            return

        var item = slot.get_children()[1]
        var texture = item.find_node("Sprite", true, false).texture
        if !texture:
            return

        dragging = Sprite.new()
        dragging.texture = texture
        dragging.modulate.a = 0.5
        add_child(dragging)
        Input.set_mouse_mode(Input.MOUSE_MODE_HIDDEN)
        return

    if !dragging:
        return

    if !event.is_pressed():
        remove_child(dragging)
        dragging = null
        var pos = get_viewport().get_mouse_position()
        get_viewport().warp_mouse(Vector2(pos.x + 1, pos.y + 1))
        yield(get_tree().create_timer(0.01), "timeout")
        Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
        return


func _process(_delta):
    if !dragging:
        return

    var pos = get_viewport().get_mouse_position()
    dragging.position = Vector2(pos.x, pos.y)


func toggle_inventory():
    var inv = get_node("Main")
    inv.visible = !inv.visible


func hide_inventory():
    var inv = get_node("Main")
    inv.visible = false


func is_visible():
    return get_node("Main").visible


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
    var grid = slot.find_parent("Main")
    if grid && !grid.visible:
        return false

    return !slot.occupied


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
