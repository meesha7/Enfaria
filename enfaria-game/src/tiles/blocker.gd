tool

extends "res://src/tiles/tile.gd"
class_name Blocker

func _init():
	tile_name = "Blocker"
	collision_layer = 2
	sprite_path = "res://assets/blocker.png"
