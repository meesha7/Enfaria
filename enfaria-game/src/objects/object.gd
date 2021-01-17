tool

extends RigidBody2D
class_name GameObject
var type = "object"

var object_name = "undefined"
var sprite_path = "res://assets/unknown.png"

func _ready():
    name = object_name
    mode = MODE_STATIC

    var sprite = Sprite.new()
    sprite.texture = load(sprite_path)
    sprite.position = Vector2(16, 16)

    add_child(sprite)
    set_meta("type", type)
    set_meta("name", name)


func serialize():
    var properties = []

    for meta in get_meta_list():
        properties.append(meta)
        properties.append(get_meta(meta))

    return properties
