tool

extends Node2D
class_name Item

var item_name = "undefined"
var sprite_path = ""

const scene = preload("res://src/objects/item.tscn")

func _ready():
    var node = scene.instance()
    if sprite_path:
        node.get_node("Sprite").texture = load(sprite_path)
    add_child(node)
