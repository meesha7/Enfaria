extends Control

func _input(event):
	if !(event is InputEventKey):
		return
	if !event.is_pressed():
		return
	var inventory = get_node("Inventory")
	if event.scancode == KEY_ESCAPE:
		var popup = get_node("Pause/Popup")
		if inventory.visible:
			inventory.hide()
		elif popup.visible:
			popup.hide()
		else:
			popup.show()
	if event.scancode == KEY_E:
		if inventory.visible:
			inventory.hide()
		else:
			inventory.show()

func _process(_delta):
	var packets = get_node("/root/connection").receive_queue
	for index in packets.size():
		var packet = packets[index]
		var command = packet.get_command()
		if "create_tile" in command:
			var split = command.split(" ")
			var tile
			match split[4]:
				"Blocker":
					tile = Blocker.new()
				"Grass":
					tile = Grass.new()
			tile.position = Vector2(split[1], split[2])
			get_node("Map").add_child(tile)
