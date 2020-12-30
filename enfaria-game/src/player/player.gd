extends KinematicBody2D

const WALK_SPEED = 5

func _process(delta):
    var new_position = Vector2(position.x, position.y)
    
    if Input.is_action_pressed("Left"):
        new_position.x += -WALK_SPEED
    elif Input.is_action_pressed("Right"):
        new_position.x +=  WALK_SPEED
        
    if Input.is_action_pressed("Up"):
        new_position.y += -WALK_SPEED
    elif Input.is_action_pressed("Down"):
        new_position.y += WALK_SPEED

    if new_position == position:
        return
        
    get_node("/root/connection").generate_packet("move" + " " + new_position.x + " " + new_position.y)
    
