extends KinematicBody2D

const WALK_SPEED = 200

var velocity = Vector2()

func _physics_process(delta):
	if Input.is_action_pressed("Left"):
		velocity.x = -WALK_SPEED
	elif Input.is_action_pressed("Right"):
		velocity.x =  WALK_SPEED
	else:
		velocity.x = 0
		
	if Input.is_action_pressed("Up"):
		velocity.y = -WALK_SPEED
	elif Input.is_action_pressed("Down"):
		velocity.y = WALK_SPEED
	else:
		velocity.y = 0

	move_and_slide(velocity)
