extends KinematicBody2D

const WALK_SPEED = 4
var z = 0

func _physics_process(_delta):
    var destination = position
    
    if Input.is_action_pressed("Left"):
        destination.x -= WALK_SPEED
    elif Input.is_action_pressed("Right"):
        destination.x += WALK_SPEED
        
    if Input.is_action_pressed("Up"):
        destination.y -= WALK_SPEED
    elif Input.is_action_pressed("Down"):
        destination.y += WALK_SPEED
    
    if destination != position:
        var string = "move %s %s %s"
        var command = string % [destination.x, destination.y, z]
        get_node("/root/connection").generate_packet(command)
