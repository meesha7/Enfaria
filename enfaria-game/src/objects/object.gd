tool

extends RigidBody2D
class_name GameObject

var object_name = "undefined"
var sprite_path = "res://assets/unknown.png"

func _ready():
    name = object_name
    mode = MODE_STATIC

    var sprite = Sprite.new()
    sprite.name = "Sprite"
    sprite.texture = load(sprite_path)
    sprite.position = Vector2(16, 16)

    add_child(sprite)


func serialize():
    var properties = []

    properties.append("type")
    properties.append(get_class())

    properties.append("name")
    properties.append(name)

    return properties
