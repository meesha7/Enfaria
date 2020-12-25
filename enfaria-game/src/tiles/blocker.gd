tool

extends "res://src/tiles/tile.gd"
class_name Blocker

func _ready():
	collision_layer = 1
	sprite_path = "res://assets/blocker.png"
	._ready()
