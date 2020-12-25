tool

extends "res://src/tiles/tile.gd"
class_name Blocker

func _init():
	collision_layer = 1
	sprite_path = "res://assets/blocker.png"
