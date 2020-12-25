tool

extends Node2D
class_name Tile

var sprite_path = ""
var collision_layer = 0

const scene = preload("res://src/tiles/tile.tscn")

func _ready():
	var node = scene.instance()
	if sprite_path:
		var image = Image.new()
		image.load(sprite_path)
		var texture = ImageTexture.new()
		texture.create_from_image(image)
		node.get_node("Sprite").texture = texture
	if collision_layer:
		node.collision_layer = collision_layer
	add_child(node)
