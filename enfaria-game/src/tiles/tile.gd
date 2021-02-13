tool

extends StaticBody2D
class_name Tile

var tile_name = "undefined"
var sprite_path = "res://assets/unimplemented.png"

var width = 32
var height = 32

func _ready():
    var off_w = height / 2
    var off_h = height / 2

    var cshape = CollisionShape2D.new()
    cshape.name = "Shape"

    var shape = RectangleShape2D.new()
    shape.extents = Vector2(off_w, off_h)
    cshape.shape = shape

    var sprite = Sprite.new()
    sprite.name = "Sprite"
    sprite.texture = load(sprite_path)
    sprite.position = Vector2(off_w, off_h)

    add_child(cshape)
    add_child(sprite)

    add_to_group("tile")


func sprite():
    find_node("Sprite")


func shape():
    find_node("Shape")
