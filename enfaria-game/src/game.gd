extends Control

onready var inventory = get_node("Inventory")
onready var chat = get_node("Chat")
onready var pause = get_node("Pause")
onready var map = get_node("Map")
onready var player = get_node("Player")

func _ready():
    var _a = get_node("Pinger").connect("timeout", self, "_on_timeout")
    get_node("Pinger").start(5)


func _on_timeout():
    get_node("/root/connection").c_ping()


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
            var tiledata = data[1]

            var tile
            match tiledata.get("name"):
                "Blocker":
                    tile = Blocker.new()
                "Grass":
                    tile = Grass.new()

            tile.position = Vector2(position.get("x"), position.get("y"))
            tile.deserialize()

            for objdata in tiledata.get("contains"):
                var obj
                match objdata.get("name"):
                    "PotatoPlant":
                        obj = PotatoPlant.new()

                obj.position = tile.position
                obj.deserialize(objdata.get("data"))
                tile.add_child(obj)

            map.add_child(tile)

        if command.has("CreatePlayer"):
            var data = command.get("CreatePlayer")
            var position = data[0]

            player.visible = true
            player.position = Vector2(position.get("x"), position.get("y"))

        if command.has("Move"):
            var data = command.get("Move")

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
                "WateringCan":
                    item = WateringCan.new()
                "PotatoSeed":
                    item = PotatoSeed.new()

            var sname = "Slot"
            sname += str(pos)
            var slot = get_node("Inventory").find_node(sname, true, false)
            slot.occupied = true
            slot.add_child(item)

        if command.has("ChatReceive"):
            var data = command.get("ChatReceive")
            chat.add_message(data)

        if command.has("CreateObject"):
            var data = command.get("CreateObject")
            var position = Vector2(data[0].get("x"), data[0].get("y"))
            var objdata = data[1]

            var obj
            match objdata.get("name"):
                "PotatoPlant":
                    obj = PotatoPlant.new()

            var tile = map.get_tile(position)
            obj.position = tile.position
            obj.deserialize(objdata.get("data"))

            tile.add_child(obj)

    packets.clear()


func _notification(what):
    if (what == MainLoop.NOTIFICATION_WM_QUIT_REQUEST):
        get_node("/root/connection").leave()
        get_tree().quit()
