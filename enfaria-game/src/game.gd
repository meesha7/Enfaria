extends Control

func _ready():
    var _a = get_node("Pinger").connect("timeout", self, "_on_timeout")
    get_node("Pinger").start(5)


func _on_timeout():
    get_node("/root/connection").generate_packet("ping")


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
        if "create_player" in command:
            var split = command.split(" ")
            var player = preload("res://src/player/player.tscn").instance()
            player.position = Vector2(split[1], split[2])
            get_node("Player").add_child(player)
        if "move" in command:
            var split = command.split(" ")
            var player = get_node("Player/Player")
            player.position = Vector2(split[1], split[2])
            player.z = split[3]
    packets.clear()


func _notification(what):
    if (what == MainLoop.NOTIFICATION_WM_QUIT_REQUEST):
        get_node("/root/connection").leave()
        get_tree().quit()
