extends Control

onready var inventory = get_node("Main/Grid")
onready var hotbar = get_node("VBoxContainer/HBoxContainer/CenterContainer/GridContainer")
var dragging = null
var selected = 1

func _input(event):
    if (event.is_action("Slot1") \
    or event.is_action("Slot2")  \
    or event.is_action("Slot3")  \
    or event.is_action("Slot4")) \
    and event.is_pressed():
        handle_switch(event)
        return

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
        var sprite = item.get_children()[0]
        if !sprite:
            return

        dragging = Sprite.new()
        dragging.texture = sprite.texture
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


func _ready():
    add_labels()

    # Selects the first slot.
    var border = hotbar.get_child(0).find_node("Selected", true, false)
    border.visible = true


# Dynamically add labels to hotbar slots.
func add_labels():
    var num = 1
    for slot in hotbar.get_children():
        var label = Label.new()
        label.add_color_override("font_color", Color(Color.black))
        label.text = str(num)
        label.margin_top = 1
        label.margin_left = 1
        num += 1
        slot.add_child(label)


func handle_switch(event):
    var switch = selected
    if event.is_action("Slot1"):
        switch = 1
    elif event.is_action("Slot2"):
        switch = 2
    elif event.is_action("Slot3"):
        switch = 3
    elif event.is_action("Slot4"):
        switch = 4

    if switch == selected:
        return

    var previous = hotbar.get_child(selected - 1).find_node("Selected", true, false)
    previous.visible = false

    var new = hotbar.get_child(switch - 1).find_node("Selected", true, false)
    new.visible = true

    selected = switch


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
    if !("object_name" in data):
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
