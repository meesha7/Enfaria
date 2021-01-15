extends Control

var occupied = false

func _ready():
    for child in get_children():
        if "item_name" in child:
            occupied = true
            return
