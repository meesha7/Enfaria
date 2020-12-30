extends Control

var occupied = false

func _ready():
    if get_node_or_null("Item"):
        occupied = true
