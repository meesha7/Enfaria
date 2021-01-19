extends Node2D

func get_tile(pos):
    for tile in get_tiles():
        var shape = tile.find_node("Shape", true, false)
        if !shape:
            continue
        if !shape.has_point(pos):
            continue
        return tile


func get_tiles():
    return get_tree().get_nodes_in_group("tile")
