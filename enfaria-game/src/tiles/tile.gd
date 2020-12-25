tool

extends Node2D
class_name Tile

var tile_name = "undefined"
var sprite_path = ""
var collision_layer = 0

const scene = preload("res://src/tiles/tile.tscn")

func _ready():
	var node = scene.instance()
	if sprite_path:
		node.get_node("Sprite").texture = load(sprite_path)
	if collision_layer:
		node.collision_layer = collision_layer
	add_child(node)
