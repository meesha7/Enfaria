extends Control

onready var inventory = get_node("Inventory")
onready var chat = get_node("Chat")
onready var pause = get_node("Pause")

func _ready():
    var _a = get_node("Pinger").connect("timeout", self, "_on_timeout")
    get_node("Pinger").start(5)


func _on_timeout():
    get_node("/root/connection").generate_packet(Dictionary({"Ping":[]}))


func _input(event):
    if !(event is InputEventKey):
        return
    if !event.is_pressed():
        return

    if event.scancode == KEY_ESCAPE:
        if inventory.is_visible():
            inventory.hide_inventory()
            return
        if chat.is_visible():
            chat.hide_chat()
            return
        pause.toggle_pause()

    if pause.is_paused():
        return

    if chat.is_visible():
        return

    if inventory.is_visible():
        return

    if event.is_action_pressed("Chat"):
        chat.show_chat()

    if event.is_action_pressed("Inventory"):
        inventory.toggle_inventory()


func _process(_delta):
    var packets = get_node("/root/connection").receive_queue
    for index in packets.size():
        var packet = packets[index]
        var command = packet.get_command()

        if command.has("CreateTile"):
            var data = command.get("CreateTile")
            var position = data[0]
            var name = data[1]

            var tile
            match name.get("name"):
                "Blocker":
                    tile = Blocker.new()
                "Grass":
                    tile = Grass.new()

            tile.position = Vector2(position.get("x"), position.get("y"))
            get_node("Map").add_child(tile)

        if command.has("CreatePlayer"):
            var data = command.get("CreatePlayer")
            var position = data[0]

            var player = preload("res://src/player/player.tscn").instance()
            player.position = Vector2(position.get("x"), position.get("y"))

            get_node("Player").add_child(player)

        if command.has("Move"):
            var data = command.get("Move")

            var player = get_node("Player/Player")
            player.position = Vector2(data.get("x"), data.get("y"))
            player.z = data.get("z")

        if command.has("CreateItem"):
            var data = command.get("CreateItem")
            var pos = data[0]
            var name = data[1].get("name")

            var item
            match name:
                "Hoe":
                    item = Hoe.new()

            var sname = "Slot"
            sname += str(pos)
            var slot = get_node("Inventory").find_node(sname, true, false)
            slot.occupied = true
            slot.add_child(item)

        if command.has("ChatReceive"):
            var data = command.get("ChatReceive")
            chat.add_message(data)

    packets.clear()


func _notification(what):
    if (what == MainLoop.NOTIFICATION_WM_QUIT_REQUEST):
        get_node("/root/connection").leave()
        get_tree().quit()
