extends Control

onready var selected = get_node("Data/Selected")
var occupied = false

func _ready():
    for child in get_children():
        var ct = ClassType.from_object(child)
        if ct.is_type("Item"):
            occupied = true


func select():
    selected.visible = true


func unselect():
    selected.visible = false


func get_item():
    for child in get_children():
        var ct = ClassType.from_object(child)
        if ct.is_type("Item"):
            return child
