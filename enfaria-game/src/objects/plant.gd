tool

extends "res://src/objects/object.gd"
class_name Plant

var stages = 1
var stage = 1

var water = 0
var max_water = 100

var base_path = ""
var ext = ".png"

func _init():
    sprite_path = base_path + "_1" + ext


func grow():
    if stage >= stages:
        return

    stage += 1
    var new_sprite = base_path + "_" + str(stage) + ext
    find_node("Sprite", true, true).texture = load(new_sprite)


func serialize():
    var properties = .serialize()

    properties.append("stage")
    properties.append(stage)

    properties.append("water")
    properties.append(water)

    return properties
