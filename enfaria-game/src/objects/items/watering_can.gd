tool

extends "res://src/objects/items/item.gd"
class_name WateringCan

func _init():
    object_name = "Watering Can (full)"
    sprite_path = "res://assets/watering_can_full.png"
    use_time = 10
